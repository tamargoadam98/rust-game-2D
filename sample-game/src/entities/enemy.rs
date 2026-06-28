use simple_engine::assets::tileset::Tile;
use simple_engine::engine::game_context::GameContext;
use simple_engine::engine::renderer::{Renderable, Renderer};
use simple_engine::entities::actor::Actor;
use simple_engine::entities::bounds::Bounds;
use simple_engine::entities::entity::Entity;

pub struct Enemy {
    actor: Actor,
}

impl Enemy {
    pub fn new(x: f32, y: f32, speed: f32, box_size: f32, sprite: Tile) -> Self {
        Self {
            actor: Actor {
                id: Self::next_id(),
                x,
                y,
                speed,
                box_size,
                sprite: sprite.scale(box_size as u32),
            },
        }
    }

    pub fn update(
        &mut self,
        player_x: f32,
        player_y: f32,
        ctx: &GameContext,
        entity_bounds: &[Bounds],
    ) {
        let mut x = self.actor.x;
        let mut y = self.actor.y;
        let step = self.actor.speed * ctx.dt;
        if player_x < self.actor.x {
            x -= step;
        }
        if player_x > self.actor.x {
            x += step;
        }
        if player_y < self.actor.y {
            y -= step;
        }
        if player_y > self.actor.y {
            y += step;
        }

        x = x.clamp(0.0, ctx.config.width as f32 - self.actor.box_size);
        y = y.clamp(self.actor.box_size, ctx.config.height as f32);

        let new_bounds = Bounds::new(
            self.actor.id,
            x,
            y,
            self.actor.box_size,
            self.actor.box_size,
        );
        if !new_bounds.check_collisions(entity_bounds) {
            self.actor.x = x;
            self.actor.y = y;
        }
    }
}

impl Entity for Enemy {
    fn bounds(&self) -> Bounds {
        self.actor.bounds()
    }
}

impl Renderable for Enemy {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.blit_pixels_centered(
            self.actor.x.round() as usize,
            self.actor.y.round() as usize,
            &self.actor.sprite.pixels,
            self.actor.sprite.size,
            self.actor.sprite.size,
        );
    }
}
