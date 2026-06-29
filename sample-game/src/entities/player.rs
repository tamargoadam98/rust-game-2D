use simple_engine::assets::tileset::Tile;
use simple_engine::engine::game_context::GameContext;
use simple_engine::engine::renderer::{Renderable, Renderer};
use simple_engine::entities::actor::Actor;
use simple_engine::entities::bounds::Bounds;
use simple_engine::entities::entity::Entity;

use crate::entities::direction::Direction;
use crate::entities::directional_sprite::DirectionalSprite;

pub struct Player {
    actor: Actor,
    sprite: DirectionalSprite,
}

impl Player {
    pub fn new(x: f32, y: f32, speed: f32, box_size: f32, sprite: Tile, sprite_diag: Tile) -> Self {
        Self {
            actor: Actor {
                id: Self::next_id(),
                x,
                y,
                speed,
                box_size,
            },
            sprite: DirectionalSprite::new(sprite, sprite_diag, box_size, Direction::Up),
        }
    }

    pub fn update(&mut self, ctx: &GameContext, entity_bounds: &[Bounds]) {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let step = self.actor.speed * ctx.dt;
        if ctx.input.is_moving_left() {
            dx -= step;
        }
        if ctx.input.is_moving_right() {
            dx += step;
        }
        if ctx.input.is_moving_up() {
            dy -= step;
        }
        if ctx.input.is_moving_down() {
            dy += step;
        }
        self.sprite.update_direction(dx, dy);

        let x = (self.x() + dx).clamp(
            self.actor.box_size / 2.0,
            ctx.config.width as f32 - self.actor.box_size / 2.0,
        );
        let y = (self.y() + dy).clamp(
            self.actor.box_size / 2.0,
            ctx.config.height as f32 - self.actor.box_size / 2.0,
        );

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
            self.actor.x.round() as usize,
            self.actor.y.round() as usize,
        );
    }
}
