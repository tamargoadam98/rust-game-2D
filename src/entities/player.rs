use crate::engine::color;
use crate::engine::game_context::GameContext;
use crate::engine::renderer::{Renderable, Renderer};
use super::entity::Entity;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub box_size: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, speed: f32, box_size: f32) -> Self {
        Self { x, y, speed, box_size }
    }
}

impl Entity for Player {
    fn update(&mut self, ctx: &GameContext) {
        let dist = self.speed * ctx.dt;
        if ctx.input.is_moving_left() { self.x -= dist; }
        if ctx.input.is_moving_right() { self.x += dist; }
        if ctx.input.is_moving_up() { self.y -= dist; }
        if ctx.input.is_moving_down() { self.y += dist; }

        self.x = self.x.clamp(0.0, ctx.config.width as f32 - self.box_size);
        self.y = self.y.clamp(self.box_size, ctx.config.height as f32);
    }
}

impl Renderable for Player {
    fn draw(&self, renderer: &mut Renderer) {
        let x_draw = self.x.round() as usize;
        let y_draw = self.y.round() as usize;
        renderer.draw_rect(x_draw, y_draw, self.box_size as usize, self.box_size as usize, color::GREEN);
    }
}
