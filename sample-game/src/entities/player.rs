use simple_engine::assets::tileset::Tile;
use simple_engine::engine::game_context::GameContext;
use simple_engine::engine::renderer::{Renderable, Renderer};
use simple_engine::entities::bounds::Bounds;
use simple_engine::entities::entity::Entity;

pub struct Player {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    speed: f32,
    box_size: f32,
    sprite: Tile,
}

impl Player {
    pub fn new(x: f32, y: f32, speed: f32, box_size: f32, sprite: Tile) -> Self {
        Self {
            id: Self::next_id(),
            x,
            y,
            speed,
            box_size,
            sprite: sprite.scale(box_size as u32),
        }
    }

    pub fn update(&mut self, ctx: &GameContext, entity_bounds: &[Bounds]) {
        let mut x = self.x;
        let mut y = self.y;
        let step = self.speed * ctx.dt;
        if ctx.input.is_moving_left() {
            x -= step;
        }
        if ctx.input.is_moving_right() {
            x += step;
        }
        if ctx.input.is_moving_up() {
            y -= step;
        }
        if ctx.input.is_moving_down() {
            y += step;
        }

        x = x.clamp(
            self.box_size / 2.0,
            ctx.config.width as f32 - self.box_size / 2.0,
        );
        y = y.clamp(
            self.box_size / 2.0,
            ctx.config.height as f32 - self.box_size / 2.0,
        );

        let new_bounds = Bounds::new(self.id, x, y, self.box_size, self.box_size);
        if !new_bounds.check_collisions(entity_bounds) {
            self.x = x;
            self.y = y;
        }
    }
}

impl Entity for Player {
    fn get_bounds(&self) -> Bounds {
        Bounds::new(self.id, self.x, self.y, self.box_size, self.box_size)
    }
}

impl Renderable for Player {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.blit_pixels_centered(
            self.x.round() as usize,
            self.y.round() as usize,
            &self.sprite.pixels,
            self.sprite.size,
            self.sprite.size,
        );
    }
}
