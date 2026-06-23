use std::time::Instant;

use crate::color;
use crate::config::Config;
use crate::input::Input;
use crate::player::Player;
use crate::renderer::{Renderable, Renderer};

pub fn run() {
    println!("Game start");

    let config = Config::new();
    let mut renderer = Renderer::new(config.title, config.width, config.height);
    let mut input = Input::new();
    let mut last_frame = Instant::now();

    let mut player = Player::new(
        (config.width / 2) as f32,
        (config.height / 2) as f32,
        200.0,
        50.0
    );

    while renderer.is_open() {
        input.poll(renderer.window());

        renderer.clear(color::BLACK);

        let now = Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        last_frame = now;

        player.update(&input, &config, dt);
        player.draw(&mut renderer);

        renderer.present();
    }
}
