use super::config::Config;
use super::key_state::KeyState;

/// Per-frame context passed to `Game::update`. Holds engine state for the current frame.
pub struct GameContext<'a> {
    pub config: &'a Config,
    pub input: &'a KeyState,
    pub dt: f32,
}

impl<'a> GameContext<'a> {
    pub fn new(config: &'a Config, input: &'a KeyState, dt: f32) -> Self {
        Self { config, input, dt }
    }
}
