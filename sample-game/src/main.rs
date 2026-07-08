mod action;
mod entities;
mod game;
mod ui;

use crate::action::Action;
use crate::game::MyGame;
use simple_engine::engine::config::Config;
use simple_engine::engine::key::Key;
use simple_engine::engine::key_state::KeyState;

fn main() {
    let config = Config::new();

    let mut input = KeyState::default();
    input.bind(Key::A, Action::MoveLeft.as_str());
    input.bind(Key::ArrowLeft, Action::MoveLeft.as_str());
    input.bind(Key::D, Action::MoveRight.as_str());
    input.bind(Key::ArrowRight, Action::MoveRight.as_str());
    input.bind(Key::W, Action::MoveUp.as_str());
    input.bind(Key::ArrowUp, Action::MoveUp.as_str());
    input.bind(Key::S, Action::MoveDown.as_str());
    input.bind(Key::ArrowDown, Action::MoveDown.as_str());
    input.bind(Key::Space, Action::Boost.as_str());
    input.bind(Key::Escape, Action::Exit.as_str());

    simple_engine::engine::run(config, input, MyGame::new(config));
}
