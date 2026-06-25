use crate::entities::bounds::Bounds;

use super::config::Config;
use super::input::Input;

/// Per-frame context passed to every entity's `update`. Holds a pre-update snapshot of the world.
pub struct GameContext<'a> {
    pub config: &'a Config,
    pub input: &'a Input,
    pub dt: f32,
    pub player_x: f32,
    pub player_y: f32,
    /// Bounds of all entities as they were at the start of this frame, before any updates.
    pub entity_bounds: &'a Vec<Bounds>
}

impl<'a> GameContext<'a> {
    pub fn new(config: &'a Config, input: &'a Input, dt: f32, player_x: f32, player_y: f32, entity_bounds: &'a Vec<Bounds>) -> Self {
        Self { config, input, dt, player_x, player_y, entity_bounds }
    }
}
