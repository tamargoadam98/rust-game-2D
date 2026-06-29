use simple_engine::assets::tileset::Tile;
use simple_engine::engine::game_context::GameContext;
use simple_engine::engine::renderer::{Renderable, Renderer};
use simple_engine::entities::actor::{Actor, ActorConfig};
use simple_engine::entities::bounds::Bounds;
use simple_engine::entities::entity::Entity;

use crate::entities::direction::Direction;
use crate::entities::directional_sprite::DirectionalSprite;

pub struct Enemy {
    actor: Actor,
    sprite: DirectionalSprite,
}

impl Enemy {
    pub fn new(x: f32, y: f32, config: ActorConfig, sprite: Tile, sprite_diag: Tile) -> Self {
        let box_size = config.box_size;
        Self {
            actor: Actor::new(Self::next_id(), x, y, config),
            sprite: DirectionalSprite::new(sprite, sprite_diag, box_size, Direction::Down),
        }
    }

    pub fn update(
        &mut self,
        player_x: f32,
        player_y: f32,
        ctx: &GameContext,
        entity_bounds: &[Bounds],
    ) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if player_x < self.actor.x - 5.0 {
            dx -= 1.0;
        } else if player_x > self.actor.x + 5.0 {
            dx += 1.0;
        }
        if player_y < self.actor.y - 5.0 {
            dy -= 1.0;
        } else if player_y > self.actor.y + 5.0 {
            dy += 1.0;
        }

        self.actor.apply_input(dx, dy, ctx.dt);
        self.sprite.update_direction(self.actor.vx, self.actor.vy);

        let x = (self.x() + self.actor.vx * ctx.dt)
            .clamp(0.0, ctx.config.width as f32 - self.actor.box_size);
        let y = (self.y() + self.actor.vy * ctx.dt)
            .clamp(self.actor.box_size, ctx.config.height as f32);

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

    pub fn x(&self) -> f32 {
        self.actor.x
    }

    pub fn y(&self) -> f32 {
        self.actor.y
    }
}

impl Entity for Enemy {
    fn bounds(&self) -> Bounds {
        self.actor.bounds()
    }
}

impl Renderable for Enemy {
    fn draw(&self, renderer: &mut Renderer) {
        self.sprite.draw(
            renderer,
            self.actor.x.round() as usize,
            self.actor.y.round() as usize,
        );
    }
}
