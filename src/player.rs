use crate::renderer::{Renderable, Renderer};

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub half_size: f32,
}

impl Player {
    pub fn new() -> Self {
        todo!()
    }

    pub fn update(&mut self, dt: f32) {
        todo!()
    }
}

impl Renderable for Player {
    fn draw(&self, renderer: &mut Renderer) {
        todo!()
    }
}
