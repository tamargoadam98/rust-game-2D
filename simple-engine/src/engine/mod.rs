pub mod color;
pub mod config;
pub mod game_context;
pub mod input;
pub mod renderer;

use std::time::Instant;

use self::config::Config;
use self::game_context::GameContext;
use self::input::Input;
use self::renderer::Renderer;

pub trait Game {
    fn update(&mut self, ctx: &GameContext);
    fn draw(&self, renderer: &mut Renderer);
}

pub fn run(config: Config, mut game: impl Game) {
    let mut renderer = Renderer::new(config.title, config.width, config.height);
    let mut input = Input::new();
    let mut last_frame = Instant::now();

    while renderer.is_open() {
        input.poll(renderer.window());

        let now = Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        last_frame = now;

        let ctx = GameContext::new(&config, &input, dt);
        game.update(&ctx);

        renderer.clear(color::BLACK);
        game.draw(&mut renderer);
        renderer.present();
    }
}
