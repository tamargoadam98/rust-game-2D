use crate::engine::color;
use crate::engine::game_context::GameContext;
use crate::engine::renderer::{Renderable, Renderer};
use crate::entities::bounds::Bounds;
use super::entity::Entity;

pub struct Player {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub box_size: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, speed: f32, box_size: f32) -> Self {
        Self { id: Self::next_id(), x, y, speed, box_size }
    }
}

impl Entity for Player {
    fn update(&mut self, ctx: &GameContext) {
        let mut x = self.x;
        let mut y = self.y;
        let step = self.speed * ctx.dt;
        if ctx.input.is_moving_left() { x -= step; }
        if ctx.input.is_moving_right() { x += step; }
        if ctx.input.is_moving_up() { y -= step; }
        if ctx.input.is_moving_down() { y += step; }

        x = x.clamp(self.box_size / 2.0, ctx.config.width as f32 - (self.box_size/ 2.0));
        y = y.clamp(self.box_size / 2.0, ctx.config.height as f32 - (self.box_size/ 2.0));

        let new_bounds = Bounds::new(self.id, x, y, self.box_size, self.box_size);
        if !new_bounds.check_collisions(ctx.entity_bounds) {
            self.x = x;
            self.y = y;
        }
    }

    fn get_bounds(&self) -> Bounds {
        Bounds::new(self.id, self.x, self.y, self.box_size, self.box_size)
    }
}

impl Renderable for Player {
    fn draw(&self, renderer: &mut Renderer) {
        let x_draw = self.x.round() as usize;
        let y_draw = self.y.round() as usize;
        renderer.draw_rect(x_draw, y_draw, self.box_size as usize, self.box_size as usize, color::GREEN);
    }
}
