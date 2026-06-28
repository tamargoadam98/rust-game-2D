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

    /// Blits an RGBA pixel buffer with `(x, y)` as the top-left corner.
    pub fn blit_pixels(&mut self, x: usize, y: usize, pixels: &[u8], width: usize, height: usize) {
        self.blit_raw(x as i32, y as i32, width, height, pixels);
    }

    /// Blits an RGBA pixel buffer centered on `(x, y)`.
    pub fn blit_pixels_centered(
        &mut self,
        x: usize,
        y: usize,
        pixels: &[u8],
        width: usize,
        height: usize,
    ) {
        let start_x = x as i32 - (width / 2) as i32;
        let start_y = y as i32 - (height / 2) as i32;
        self.blit_raw(start_x, start_y, width, height, pixels);
    }

    fn blit_raw(&mut self, start_x: i32, start_y: i32, width: usize, height: usize, pixels: &[u8]) {
        let mut i = 0;
        for row in 0..height as i32 {
            for col in 0..width as i32 {
                let x = start_x + col;
                let y = start_y + row;
                if x >= 0 && y >= 0 {
                    self.draw_pixel_rgba(
                        x as usize,
                        y as usize,
                        pixels[i],
                        pixels[i + 1],
                        pixels[i + 2],
                        pixels[i + 3],
                    );
                }
                i += 4;
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
