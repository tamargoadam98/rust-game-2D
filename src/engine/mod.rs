pub mod color;
pub mod config;
pub mod game_context;
pub mod input;
pub mod renderer;

use std::time::Instant;

use crate::entities::enemy::Enemy;
use crate::entities::entity::Entity;
use crate::entities::player::Player;
use self::config::Config;
use self::game_context::GameContext;
use self::input::Input;
use self::renderer::{Renderable, Renderer};

pub fn run() {
    println!("Game start");

    let config = Config::new();
    let mut renderer = Renderer::new(config.title, config.width, config.height);
    let mut input = Input::new();
    let mut last_frame = Instant::now();

    let mut entities: Vec<Box<dyn Entity>> = Vec::new();
    let mut player = Player::new((config.width / 2) as f32, (config.height / 2) as f32, 200.0, 50.0);
    entities.push(Box::new(Enemy::new(0.0, 0.0, 100.0, 25.0)));
    entities.push(Box::new(Enemy::new((config.width - 25) as f32, 0.0, 100.0, 25.0)));

    while renderer.is_open() {
        input.poll(renderer.window());

        renderer.clear(color::BLACK);

        let now = Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        last_frame = now;

        let ctx = GameContext::new(&config, &input, dt, player.x, player.y);
        player.update(&ctx);
        player.draw(&mut renderer);
        for entity in &mut entities {
            entity.update(&ctx);
            entity.draw(&mut renderer);
        }

        renderer.present();
    }
}
