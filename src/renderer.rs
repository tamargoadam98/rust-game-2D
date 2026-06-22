use minifb::{Key, Window, WindowOptions};

pub trait Renderable {
    fn draw(&self, renderer: &mut Renderer);
}

pub struct Renderer {
    buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    window: Window,
}

impl Renderer {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let window = Window::new(title, width, height, WindowOptions::default())
            .unwrap_or_else(|e| panic!("Failed to create window: {}", e));

        Self {
            buffer: vec![0; width * height],
            width,
            height,
            window,
        }
    }

    pub fn is_open(&self) -> bool {
       self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color)
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        self.buffer[y * self.width + x] = color
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for y_coord in y - height..y {
            for x_coord in x..x + width {
                self.draw_pixel(x_coord, y_coord, color);
            }
        }
    }

    pub fn present(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }
}
