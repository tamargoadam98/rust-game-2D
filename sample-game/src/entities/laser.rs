use simple_engine::{
    assets::tileset::Tile,
    engine::{
        game_context::GameContext,
        renderer::{Renderable, Renderer},
    },
    entities::{
        actor::{Actor, ActorConfig},
        bounds::Bounds,
        entity::Entity,
    },
};

use crate::entities::{direction::Direction, directional_sprite::DirectionalSprite};

#[derive(Clone)]
pub struct Laser {
    actor: Actor,
    sprite: DirectionalSprite,
    origin: (f32, f32),
    fire_distance: f32,
    owner_bounds_id: u32,
}

impl Laser {
    pub fn new(
        x: f32,
        y: f32,
        config: ActorConfig,
        sprite: Tile,
        sprite_diag: Tile,
        direction: Direction,
        fire_distance: f32,
        owner_bounds_id: u32,
    ) -> Self {
        let box_size = config.box_size;
        Self {
            actor: Actor::new(Self::next_id(), x, y, config),
            sprite: DirectionalSprite::new(sprite, sprite_diag, box_size, direction),
            origin: (x, y),
            fire_distance,
            owner_bounds_id,
        }
    }

    /// Advances the laser. Returns `false` if it hit something or exceeded `fire_distance`.
    pub fn update(&mut self, ctx: &GameContext, entity_bounds: &[Bounds]) -> bool {
        self.actor.x += self.actor.vx * ctx.dt;
        self.actor.y += self.actor.vy * ctx.dt;

        let new_bounds = Bounds::new(
            self.actor.id,
            self.actor.x,
            self.actor.y,
            self.actor.box_size,
            self.actor.box_size,
        );

        if new_bounds.check_collisions_excluding(entity_bounds, self.owner_bounds_id) {
            return false;
        }

        let distance = ((self.actor.x - self.origin.0).powi(2)
            + (self.actor.y - self.origin.1).powi(2))
        .sqrt();
        distance <= self.fire_distance
    }

    fn get_direction_deltas(&self) -> (f32, f32) {
        const DIAG: f32 = std::f32::consts::FRAC_1_SQRT_2;
        match self.sprite.direction {
            Direction::Up => (0.0, -1.0),
            Direction::UpRight => (DIAG, -DIAG),
            Direction::Right => (1.0, 0.0),
            Direction::DownRight => (DIAG, DIAG),
            Direction::Down => (0.0, 1.0),
            Direction::DownLeft => (-DIAG, DIAG),
            Direction::Left => (-1.0, 0.0),
            Direction::UpLeft => (-DIAG, -DIAG),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.actor.x = x;
        self.actor.y = y;
        self.origin = (x, y)
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.sprite.direction = direction;
        let (dx, dy) = self.get_direction_deltas();
        self.actor.vx = dx * self.actor.max_speed;
        self.actor.vy = dy * self.actor.max_speed;
    }
}

impl Entity for Laser {
    fn bounds(&self) -> Bounds {
        self.actor.bounds()
    }
}

impl Renderable for Laser {
    fn draw(&self, renderer: &mut Renderer) {
        self.sprite.draw(
            renderer,
            self.actor.x.round() as i32,
            self.actor.y.round() as i32,
        );
    }
}
