use super::config::Config;
use super::input::Input;

/// Per-frame context passed to `Game::update`. Holds engine state for the current frame.
pub struct GameContext<'a> {
    pub config: &'a Config,
    pub input: &'a Input,
    pub dt: f32,
}

impl<'a> GameContext<'a> {
    pub fn new(config: &'a Config, input: &'a Input, dt: f32) -> Self {
        Self { config, input, dt }
    }
}
