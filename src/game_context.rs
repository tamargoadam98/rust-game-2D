use crate::{config::Config, input::Input, player};

pub struct GameContext<'a> {
    pub config: &'a Config,
    pub input: &'a Input,
    pub dt: f32,
    pub player_x: f32,
    pub player_y: f32,
}

impl<'a> GameContext<'a> {
    pub fn new(config: &'a Config, input: &'a Input, dt: f32, player_x: f32, player_y: f32) -> Self {
        Self { config, input, dt,  player_x, player_y }
    }
}
