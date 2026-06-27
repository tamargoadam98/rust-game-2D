use simple_engine::assets::tileset::Tile;
use simple_engine::engine::game_context::GameContext;
use simple_engine::engine::renderer::{Renderable, Renderer};
use simple_engine::entities::bounds::Bounds;
use simple_engine::entities::entity::Entity;

pub struct Enemy {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    speed: f32,
    box_size: f32,
    sprite: Tile,
}

impl Enemy {
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

    pub fn update(
        &mut self,
        player_x: f32,
        player_y: f32,
        ctx: &GameContext,
        entity_bounds: &[Bounds],
    ) {
        let mut x = self.x;
        let mut y = self.y;
        let step = self.speed * ctx.dt;
        if player_x < self.x {
            x -= step;
        }
        if player_x > self.x {
            x += step;
        }
        if player_y < self.y {
            y -= step;
        }
        if player_y > self.y {
            y += step;
        }

        x = x.clamp(0.0, ctx.config.width as f32 - self.box_size);
        y = y.clamp(self.box_size, ctx.config.height as f32);

        let new_bounds = Bounds::new(self.id, x, y, self.box_size, self.box_size);
        if !new_bounds.check_collisions(entity_bounds) {
            self.x = x;
            self.y = y;
        }
    }
}

impl Entity for Enemy {
    fn get_bounds(&self) -> Bounds {
        Bounds::new(self.id, self.x, self.y, self.box_size, self.box_size)
    }
}

impl Renderable for Enemy {
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
