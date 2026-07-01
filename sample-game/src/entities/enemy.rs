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
        let diff_x = player_x - self.actor.x;
        let diff_y = player_y - self.actor.y;
        let dist = (diff_x * diff_x + diff_y * diff_y).sqrt();
        let (dx, dy) = if dist > 5.0 {
            (diff_x / dist, diff_y / dist)
        } else {
            (0.0, 0.0)
        };

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
