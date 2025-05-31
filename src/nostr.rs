use nostr_sdk::{Client, EventBuilder, Filter, Keys, Kind, RelayPoolNotification, Tag, Alphabet, TagKind};
use tokio::sync::mpsc::Sender;

pub async fn init_client() -> anyhow::Result<(Client, Keys)> {
    let my_keys = Keys::generate();
    let client = Client::new(&my_keys);
    client.add_relay("wss://relay.snort.social").await?;
    client.connect().await;

    Ok((client, my_keys))
}

pub async fn subscribe_positions(
    client: Client,
    self_pubkey: String,
    tx: Sender<(String, f32, f32)>,
) {
    let filter = Filter::new()
        .kind(Kind::Ephemeral(20009))
        .since(nostr_sdk::Timestamp::now());

    // Fix 1: Provide the second argument explicitly
    client.subscribe(vec![filter], None).await;

    // Spawn a listener task
    tokio::spawn(async move {
        let mut notifications = client.notifications();

        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event {
                event, relay_url, ..
            } = notification
            {
                println!("📩 Event received from relay: {}", relay_url);
                println!("🆔 Event ID: {}", event.id);
                println!("👤 Pubkey: {}", event.pubkey);
                println!("🗂 Kind: {:?}", event.kind);
                println!("📝 Content: {}", event.content);
                println!("🏷 Tags: {:?}", event.tags);

                // Skip events from self
                if event.pubkey.to_string() == self_pubkey {
                    continue;
                }

                let mut x = None;
                let mut y = None;

                for tag in &event.tags {
                    println!("➡ Tag: {:?}", tag);

                    if let Tag::Generic(tag_kind, values) = tag {
                        println!("   - Values: {:?}", values);

                        match tag_kind {
                            TagKind::SingleLetter(sl) if sl.character == Alphabet::X => {
                                x = values.get(1).and_then(|s| s.parse().ok());
                                println!("   ✅ Parsed x = {:?}", x);
                            }
                            TagKind::SingleLetter(sl) if sl.character == Alphabet::Y => {
                                y = values.get(1).and_then(|s| s.parse().ok());
                                println!("   ✅ Parsed y = {:?}", y);
                            }
                            _ => {
                                println!("   ⚠️ Unhandled tag kind: {:?}", tag_kind);
                            }
                        }
                    }
                }

                if let (Some(px), Some(py)) = (x, y) {
                    let _ = tx.send((event.pubkey.to_string(), px, py)).await;
                }
            } else {
                //println!("📬 Other notification: {:?}", notification);
            }
        }
    });
}

pub async fn send_position(client: &Client, my_keys: &Keys, x: f32, y: f32) {
    let tags = vec![
        Tag::Generic("x".into(), vec!["x".into(), x.to_string()]),
        Tag::Generic("y".into(), vec!["y".into(), y.to_string()]),
    ];

    let event = EventBuilder::new(Kind::Ephemeral(20009), "", tags)
        .to_event(my_keys)
        .unwrap();

    match client.send_event(event).await {
        Ok(event_id) => println!("✅ Event sent! ID: {}", event_id),
        Err(e) => eprintln!("❌ Failed to send event: {:?}", e),
    }
}
