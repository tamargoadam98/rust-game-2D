use pixels::Pixels;

pub trait Renderable {
    fn draw(&self, renderer: &mut Renderer);
}

pub struct Renderer {
    pixels: Pixels<'static>,
    pub width: usize,
    pub height: usize,
}

impl Renderer {
    pub(crate) fn new(pixels: Pixels<'static>, width: usize, height: usize) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: u32) {
        let [r, g, b] = rgb_bytes(color);
        for px in self.pixels.frame_mut().chunks_exact_mut(4) {
            px[0] = r;
            px[1] = g;
            px[2] = b;
            px[3] = 0xff;
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let i = (y * self.width + x) * 4;
            let [r, g, b] = rgb_bytes(color);
            let frame = self.pixels.frame_mut();
            frame[i] = r;
            frame[i + 1] = g;
            frame[i + 2] = b;
            frame[i + 3] = 0xff;
        }
    }

    /// Draws a pixel blended over whatever is already in the buffer.
    /// Uses standard "src over dst" alpha compositing:
    ///   out = (src * a + dst * (255 - a)) / 255
    pub fn draw_pixel_rgba(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8, a: u8) {
        if a == 0 || x >= self.width || y >= self.height {
            return;
        }
        let i = (y * self.width + x) * 4;
        let frame = self.pixels.frame_mut();
        if a == 255 {
            frame[i] = r;
            frame[i + 1] = g;
            frame[i + 2] = b;
            frame[i + 3] = 0xff;
        } else {
            let a = a as u32;
            let inv = 255 - a;
            frame[i] = ((r as u32 * a + frame[i] as u32 * inv) / 255) as u8;
            frame[i + 1] = ((g as u32 * a + frame[i + 1] as u32 * inv) / 255) as u8;
            frame[i + 2] = ((b as u32 * a + frame[i + 2] as u32 * inv) / 255) as u8;
            frame[i + 3] = 0xff;
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

    /// Blits a tile's RGBA pixel buffer to screen position `(x, y)`.
    pub fn blit_tile(&mut self, x: usize, y: usize, pixels: &[u8], tile_size: usize) {
        for row in 0..tile_size {
            for col in 0..tile_size {
                let i = (row * tile_size + col) * 4;
                self.draw_pixel_rgba(
                    x + col,
                    y + row,
                    pixels[i],
                    pixels[i + 1],
                    pixels[i + 2],
                    pixels[i + 3],
                );
            }
        }
    }

    pub(crate) fn present(&mut self) {
        self.pixels.render().unwrap();
    }
}

fn rgb_bytes(color: u32) -> [u8; 3] {
    [
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    ]
}
