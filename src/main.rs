mod game;
mod nostr;

use game::Player;
use macroquad::{ prelude::*};
use nostr::{init_client, send_position};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::{runtime::Runtime, sync::mpsc};
use macroquad::conf::Conf;

fn window_conf() -> Conf {
    Conf {
        // Set window options via miniquad_conf
        miniquad_conf: miniquad::conf::Conf {
            window_title: "Club Ostrich".to_string(),
            window_width: 800,
            window_height: 600,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let (client, my_keys) = rt
        .block_on(init_client())
        .expect("Failed to init nostr client");

    let mut player = Player::new(200.0, 200.0, RED);
    let pubkey = my_keys.public_key().to_string();

    // Channel to receive other players' positions
    let (tx, mut rx) = mpsc::channel(100);
    rt.block_on(nostr::subscribe_positions(
        client.clone(),
        pubkey.clone(),
        tx,
    ));

    let mut last_send = Instant::now() - Duration::from_secs(1);
    let mut last_sent_x = player.x;
    let mut last_sent_y = player.y;

    // Store positions of other players
    let mut others: HashMap<String, (f32, f32)> = HashMap::new();

    loop {
        clear_background(WHITE);
        let border_x = 0.0;
        let border_y = 0.0;
        let border_width = 800.0;
        let border_height = 600.0;

        draw_rectangle_lines(border_x, border_y, border_width, border_height, 4.0, BLACK);
        // Draw local player
        player.update();
        player.draw();
        
        let my_truncated = if pubkey.len() > 8 {
            format!("{}...{}", &pubkey[..4], &pubkey[pubkey.len()-4..])
        } else {
            pubkey.to_string()
        };
        draw_text(&my_truncated, player.x + 20.0, player.y, 20.0, BLACK);

        // Draw other players
        for (pubkey, (x, y)) in others.iter() {
            draw_circle(*x, *y, 10.0, BLUE);
        
            let truncated = if pubkey.len() > 8 {
                format!("{}...{}", &pubkey[..4], &pubkey[pubkey.len()-4..])
            } else {
                pubkey.to_string()
            };
        
            draw_text(&truncated, *x + 12.0, *y, 20.0, BLACK);
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
            others.insert(pubkey.clone(), (x, y));
            println!("Updated player {} position to ({}, {})", pubkey, x, y);
            println!("Current others map: {:?}", others);
        }

        next_frame().await;
    }
}
