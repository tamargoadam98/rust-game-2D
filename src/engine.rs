use std::time::Instant;

use crate::color;
use crate::input::Input;
use crate::renderer::Renderer;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub fn run() {
    println!("Game start");

    let mut renderer = Renderer::new("Minifb Demo - Press ESC to Exit", WIDTH, HEIGHT);
    let mut input = Input::new();
    let mut last_frame = Instant::now();

    let mut x_pos: f32 = (WIDTH / 2) as f32;
    let mut y_pos: f32 = (HEIGHT / 2) as f32;
    let box_size: f32 = 50.0;
    let speed: f32 = 200.0;

    while renderer.is_open() {
        input.poll(renderer.window());

        renderer.clear(color::BLACK);

        let now = Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        last_frame = now;
        let dist = speed * dt;

        if input.is_moving_left() {
            x_pos -= dist;
        }
        if input.is_moving_right() {
            x_pos += dist;
        }
        if input.is_moving_up() {
            y_pos -= dist;
        }
        if input.is_moving_down() {
            y_pos += dist;
        }

        x_pos = x_pos.clamp(0.0, WIDTH as f32 - box_size);
        y_pos = y_pos.clamp(box_size, HEIGHT as f32);

        let x_draw = x_pos.round() as usize;
        let y_draw = y_pos.round() as usize;

        // Draw a simple box that can move around the screen
        renderer.draw_rect(x_draw, y_draw, box_size as usize, box_size as usize, color::GREEN);

        renderer.present();
    }
}
