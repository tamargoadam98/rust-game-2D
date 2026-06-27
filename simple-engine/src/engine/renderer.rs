use minifb::{Key, Window, WindowOptions};

/// Implemented by anything that knows how to draw itself to the pixel buffer.
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
        if (x as usize) < self.width
            && (y as usize) < self.height {
            self.buffer[y * self.width + x] = color
        }
    }

    /// Draws a rectangle centered on `(x, y)`. Clips silently at screen edges.
    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let half_w = (width / 2) as i32;
        let half_h = (height / 2) as i32;
        let cx = x as i32;
        let cy = y as i32;

        for y_coord in cy - half_h..cy + half_h {
            for x_coord in cx - half_w..cx + half_w {
                if x_coord >= 0 && y_coord >= 0 {
                    self.draw_pixel(x_coord as usize, y_coord as usize, color);
                }
            }
        }
    }

    /// Blits (block-transfers) a tile's pixel buffer to screen position `(x, y)` in pixel coordinates.
    pub fn blit_tile(&mut self, x: usize, y: usize, pixels: &[u32], tile_size: usize) {
        let mut i = 0;
        for y_coord in y .. y + tile_size {
            for x_coord in x .. x + tile_size {
                self.draw_pixel(x_coord, y_coord, pixels[i]);
                i += 1;
            }
        }
    }

    /// Flushes the pixel buffer to the window. Call once at the end of each frame.
    pub fn present(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }
}
