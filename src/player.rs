use crate::{color, config::Config, input::Input, renderer::{Renderable, Renderer}};

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub box_size: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, speed: f32, box_size: f32) -> Self {
        Self {
            x,
            y,
            speed,
            box_size
        }
    }

    pub fn update(&mut self, input: &Input, config: &Config, dt: f32) {
        let dist = self.speed * dt;
        if input.is_moving_left() {
            self.x -= dist;
        }
        if input.is_moving_right() {
            self.x += dist;
        }
        if input.is_moving_up() {
            self.y -= dist;
        }
        if input.is_moving_down() {
            self.y += dist;
        }

        self.x = self.x.clamp(0.0, config.width as f32 - self.box_size);
        self.y = self.y.clamp(self.box_size, config.height as f32);
    }
}

impl Renderable for Player {
    fn draw(&self, renderer: &mut Renderer) {
        let x_draw = self.x.round() as usize;
        let y_draw = self.y.round() as usize;

        // Draw a simple box that can move around the screen
        renderer.draw_rect(x_draw, y_draw, self.box_size as usize, self.box_size as usize, color::GREEN);
    }
}
