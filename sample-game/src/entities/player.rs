use simple_engine::assets::tileset::Tile;
use simple_engine::engine::game_context::GameContext;
use simple_engine::engine::renderer::{Renderable, Renderer};
use simple_engine::entities::actor::{Actor, ActorConfig};
use simple_engine::entities::bounds::Bounds;
use simple_engine::entities::entity::Entity;

use crate::action::Action;
use crate::entities::direction::Direction;
use crate::entities::directional_sprite::DirectionalSprite;

pub struct Player {
    actor: Actor,
    sprite: DirectionalSprite,
}

impl Player {
    pub fn new(x: f32, y: f32, config: ActorConfig, sprite: Tile, sprite_diag: Tile) -> Self {
        let box_size = config.box_size;
        Self {
            actor: Actor::new(Self::next_id(), x, y, config),
            sprite: DirectionalSprite::new(sprite, sprite_diag, box_size, Direction::Up),
        }
    }

    pub fn update(&mut self, ctx: &GameContext, entity_bounds: &[Bounds]) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if ctx.input.is_active(Action::MoveLeft.as_str()) {
            dx -= 1.0;
        } else if ctx.input.is_active(Action::MoveRight.as_str()) {
            dx += 1.0;
        }
        if ctx.input.is_active(Action::MoveUp.as_str()) {
            dy -= 1.0;
        } else if ctx.input.is_active(Action::MoveDown.as_str()) {
            dy += 1.0;
        }

        self.actor.apply_input(dx, dy, ctx.dt);
        self.sprite.update_direction(self.actor.vx, self.actor.vy);

        let x = self.x() + self.actor.vx * ctx.dt;
        let y = self.y() + self.actor.vy * ctx.dt;

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

impl Entity for Player {
    fn bounds(&self) -> Bounds {
        self.actor.bounds()
    }
}

impl Renderable for Player {
    fn draw(&self, renderer: &mut Renderer) {
        self.sprite.draw(
            renderer,
            self.actor.x.round() as i32,
            self.actor.y.round() as i32,
        );
    }
}
