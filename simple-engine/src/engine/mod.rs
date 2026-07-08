pub mod camera;
pub mod color;
pub mod config;
pub mod game_context;
pub mod key;
pub mod key_state;
pub mod renderer;

use std::sync::Arc;
use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use crate::engine::camera::Camera;

use self::config::Config;
use self::game_context::GameContext;
use self::key::Key;
use self::key_state::KeyState;
use self::renderer::Renderer;

pub trait Game {
    fn update(&mut self, ctx: &GameContext);
    fn draw(&self, renderer: &mut Renderer);
}

struct App<G: Game> {
    config: Config,
    game: G,
    input: KeyState,
    last_frame: Instant,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl<G: Game> ApplicationHandler for App<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(self.config.title)
                        .with_inner_size(winit::dpi::LogicalSize::new(
                            self.config.width as f64,
                            self.config.height as f64,
                        )),
                )
                .unwrap(),
        );
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, Arc::clone(&window));
        let pixels =
            Pixels::new(self.config.width as u32, self.config.height as u32, surface).unwrap();
        self.renderer = Some(Renderer::new(
            pixels,
            Camera::default(),
            self.config.width,
            self.config.height,
        ));
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(key) = Key::from_winit(&event.logical_key) {
                    match event.state {
                        ElementState::Pressed => self.input.key_down(key),
                        ElementState::Released => self.input.key_up(key),
                    }
                }
                if self.input.is_active("exit") {
                    event_loop.exit();
                }
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = (now - self.last_frame).as_secs_f32();
                self.last_frame = now;

                let ctx = GameContext::new(&self.config, &self.input, dt);
                self.game.update(&ctx);

                if let Some(renderer) = self.renderer.as_mut() {
                    self.game.draw(renderer);
                    renderer.present();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

pub fn run(config: Config, input: KeyState, game: impl Game) {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        config,
        game,
        input,
        last_frame: Instant::now(),
        window: None,
        renderer: None,
    };
    event_loop.run_app(&mut app).unwrap();
}
