use macroquad::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

impl Player {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self { x, y, color }
    }

    pub fn update(&mut self) {
        const SPEED: f32 = 5.0;
        if is_key_down(KeyCode::Right) {
            self.x += SPEED;
        }
        if is_key_down(KeyCode::Left) {
            self.x -= SPEED;
        }
        if is_key_down(KeyCode::Down) {
            self.y += SPEED;
        }
        if is_key_down(KeyCode::Up) {
            self.y -= SPEED;
        }
        self.x = self.x.clamp(0.0, 800.0);
        self.y = self.y.clamp(0.0, 600.0);
    }

    pub fn draw(&self) {
        draw_rectangle(self.x - 15.0, self.y - 15.0, 30.0, 30.0, self.color);
    }
}
