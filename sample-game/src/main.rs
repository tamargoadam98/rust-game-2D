mod color;
mod entities;
mod game;

use simple_engine::engine::config::Config;
use crate::game::MyGame;

fn main() {
    let config = Config::new();
    simple_engine::engine::run(config, MyGame::new(config));
}
