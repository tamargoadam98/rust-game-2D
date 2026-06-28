mod entities;
mod game;

use crate::game::MyGame;
use simple_engine::engine::config::Config;

fn main() {
    let config = Config::new();
    simple_engine::engine::run(config, MyGame::new(config));
}
