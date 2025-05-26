mod game;
mod nostr;

use game::Player;
use macroquad::prelude::*;
use nostr::{init_client, send_position};
use std::{collections::HashMap, time::{Duration, Instant}};
use tokio::{runtime::Runtime, sync::mpsc};

#[macroquad::main("Club Ostrich")]
async fn main() {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let (client, my_keys) = rt.block_on(init_client()).expect("Failed to init nostr client");

    let mut player = Player::new(200.0, 200.0, RED);
    let pubkey = my_keys.public_key().to_string();

    // Channel to receive other players' positions
    let (tx, mut rx) = mpsc::channel(100);
    rt.block_on(nostr::subscribe_positions(client.clone(), pubkey.clone(), tx));

    let mut last_send = Instant::now() - Duration::from_secs(1);
    let mut last_sent_x = player.x;
    let mut last_sent_y = player.y;

    // Store positions of other players
    let mut others: HashMap<String, (f32, f32)> = HashMap::new();

    loop {
        clear_background(WHITE);

        // Draw local player
        player.update();
        player.draw();

        // Draw other players
        for (_pubkey, (x, y)) in others.iter() {
            draw_circle(*x, *y, 10.0, BLUE);
        }

        // Throttled position update
        if player.x != last_sent_x || player.y != last_sent_y {
            if last_send.elapsed() >= Duration::from_secs(1) {
                let client_clone = client.clone();
                let keys_clone = my_keys.clone();
                let x = player.x;
                let y = player.y;

                rt.spawn(async move {
                    send_position(&client_clone, &keys_clone, x, y).await;
                });

                last_sent_x = x;
                last_sent_y = y;
                last_send = Instant::now();
            }
        }

        // Handle incoming positions
        while let Ok((pubkey, x, y)) = rx.try_recv() {
            others.insert(pubkey, (x, y));
        }

        next_frame().await;
    }
}
