use nostr_sdk::{Client, EventBuilder, Filter, Keys, Kind, RelayPoolNotification, Tag};
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
    client
        .subscribe(vec![filter], None)
        .await;

    // Spawn a listener task
    tokio::spawn(async move {
        let mut notifications = client.notifications();

        while let Ok(notification) = notifications.recv().await {
            // Fix 2: Updated pattern match syntax for RelayPoolNotification
            if let RelayPoolNotification::Event { event, .. } = notification {
                if event.pubkey.to_string() == self_pubkey {
                    continue;
                }

                // Fix 3: TagKind does not have `as_str()` — use matching directly
                let mut x = None;
                let mut y = None;

                for tag in &event.tags {
                    if let Tag::Generic(tag_kind, values) = tag {
                        match tag_kind {
                            // TagKind is an enum — match variants directly
                            nostr_sdk::TagKind::Custom(t) if t == "x" => {
                                x = values.get(0).and_then(|s| s.parse().ok())
                            }
                            nostr_sdk::TagKind::Custom(t) if t == "y" => {
                                y = values.get(0).and_then(|s| s.parse().ok())
                            }
                            _ => {}
                        }
                    }
                }

                if let (Some(px), Some(py)) = (x, y) {
                    let _ = tx.send((event.pubkey.to_string(), px, py)).await;
                }
            }
        }
    });
}

pub async fn send_position(client: &Client, my_keys: &Keys, x: f32, y: f32) {
    let tags = vec![
        Tag::Generic("coord".into(), vec!["x".into(), x.to_string()]),
        Tag::Generic("coord".into(), vec!["y".into(), y.to_string()]),
    ];

    let event = EventBuilder::new(Kind::Ephemeral(20009), "", tags)
        .to_event(my_keys)
        .unwrap();

    match client.send_event(event).await {
        Ok(event_id) => println!("✅ Event sent! ID: {}", event_id),
        Err(e) => eprintln!("❌ Failed to send event: {:?}", e),
    }
}
