use simple_engine::assets::tileset::Tile;
use simple_engine::engine::game_context::GameContext;
use simple_engine::engine::renderer::{Renderable, Renderer};
use simple_engine::entities::actor::{Actor, ActorConfig};
use simple_engine::entities::bounds::Bounds;
use simple_engine::entities::entity::Entity;

use crate::action::Action;
use crate::entities::direction::Direction;
use crate::entities::directional_sprite::DirectionalSprite;
use crate::entities::laser::Laser;

pub struct Player {
    actor: Actor,
    sprite: DirectionalSprite,
    base_max_speed: f32,
    base_acceleration: f32,
    boost: f32,
    laser: Laser,
    shoot_cooldown: f32,
}

impl Player {
    const MAX_BOOST: f32 = 100.0;
    const BOOST_DRAIN: f32 = 50.0;
    const BOOST_REGEN: f32 = 25.0;
    const SHOOT_COOLDOWN: f32 = 0.3;

    pub fn new(
        x: f32,
        y: f32,
        config: ActorConfig,
        sprite: Tile,
        sprite_diag: Tile,
        laser_sprite: Tile,
        laser_sprite_diag: Tile,
    ) -> Self {
        let box_size = config.box_size;
        let actor = Actor::new(Self::next_id(), x, y, config);
        let owner_id = actor.id;
        Self {
            actor,
            sprite: DirectionalSprite::new(sprite, sprite_diag, box_size, Direction::Up),
            base_max_speed: config.max_speed,
            base_acceleration: config.acceleration,
            boost: Self::MAX_BOOST,
            laser: Self::build_laser(laser_sprite, laser_sprite_diag, owner_id),
            shoot_cooldown: 0.0,
        }
    }

    fn build_laser(sprite: Tile, sprite_diag: Tile, owner_bounds_id: u32) -> Laser {
        let laser_config = ActorConfig {
            max_speed: 1500.0,
            acceleration: 1.0,
            deceleration: 0.0,
            box_size: 32.0,
        };
        Laser::new(
            0.0,
            0.0,
            laser_config,
            sprite,
            sprite_diag,
            Direction::Up,
            2048.0,
            owner_bounds_id,
        )
    }

    /// Returns a newly spawned `Laser` if the player fired this frame, otherwise `None`.
    pub fn update(&mut self, ctx: &GameContext, entity_bounds: &[Bounds]) -> Option<Laser> {
        self.update_boost(ctx);
        self.shoot_cooldown = (self.shoot_cooldown - ctx.dt).max(0.0);

        let laser = self.check_fire_laser(ctx);

        let (dx, dy) = self.get_direction_deltas(ctx);

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

        laser
    }

    fn update_boost(&mut self, ctx: &GameContext) {
        if ctx.input.is_active(Action::Boost.as_str()) && self.boost > 0.0 {
            self.actor.max_speed = self.base_max_speed * 2.0;
            self.actor.acceleration = self.base_acceleration * 3.0;
            self.boost -= Self::BOOST_DRAIN * ctx.dt;
        } else {
            self.actor.max_speed = self.base_max_speed;
            self.actor.acceleration = self.base_acceleration;
            if self.boost < Self::MAX_BOOST {
                self.boost += Self::BOOST_REGEN * ctx.dt;
            }
        }
        self.boost = self.boost.clamp(0.0, Self::MAX_BOOST);
    }

    fn check_fire_laser(&mut self, ctx: &GameContext) -> Option<Laser> {
        if self.shoot_cooldown == 0.0 && ctx.input.is_active(Action::Shoot.as_str()) {
            self.shoot_cooldown = Self::SHOOT_COOLDOWN;
            let mut new_laser = self.laser.clone(); // todo: consider refactor to avoid need for deep copy
            new_laser.set_direction(self.sprite.direction);
            new_laser.set_position(self.x(), self.y());
            Some(new_laser)
        } else {
            None
        }
    }

    fn get_direction_deltas(&self, ctx: &GameContext) -> (f32, f32) {
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
        (dx, dy)
    }

    pub fn get_boost_percentage(&self) -> f32 {
        self.boost / Self::MAX_BOOST
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
