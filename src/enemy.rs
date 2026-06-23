use crate::{color, config::Config, entity::Entity, game_context::GameContext, input::Input, renderer::{Renderable, Renderer}};

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub box_size: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, speed: f32, box_size: f32) -> Self {
        Self {
            x,
            y,
            speed,
            box_size
        }
    }
}

impl Entity for Enemy {
    fn update(&mut self, ctx: &GameContext) {
        let dist = self.speed * ctx.dt;
        if ctx.player_x < self.x {
            self.x -= dist;
        }
        if ctx.player_x > self.x {
            self.x += dist;
        }
        if ctx.player_y < self.y {
            self.y -= dist;
        }
        if ctx.player_y > self.y {
            self.y += dist;
        }

        self.x = self.x.clamp(0.0, ctx.config.width as f32 - self.box_size);
        self.y = self.y.clamp(self.box_size, ctx.config.height as f32);
    }
}

impl Renderable for Enemy {
    fn draw(&self, renderer: &mut Renderer) {
        let x_draw = self.x.round() as usize;
        let y_draw = self.y.round() as usize;

        // Draw a simple box that can move around the screen
        renderer.draw_rect(x_draw, y_draw, self.box_size as usize, self.box_size as usize, color::RED);
    }
}
