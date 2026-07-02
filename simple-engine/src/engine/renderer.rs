use pixels::Pixels;

use crate::engine::camera::Camera;

pub trait Renderable {
    fn draw(&self, renderer: &mut Renderer);
}

pub enum BlendMode {
    Opaque,
    Alpha,
}

pub struct Renderer {
    pixels: Pixels<'static>,
    pub(crate) camera: Camera,
    pub width: usize,
    pub height: usize,
}

impl Renderer {
    pub(crate) fn new(
        pixels: Pixels<'static>,
        camera: Camera,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            pixels,
            camera,
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

    pub fn blit_pixels(
        &mut self,
        x: i32,
        y: i32,
        pixels: &[u8],
        width: usize,
        height: usize,
        blend: BlendMode,
    ) {
        self.blit_raw(x, y, width, height, pixels, blend);
    }

    pub fn blit_pixels_centered(
        &mut self,
        x: i32,
        y: i32,
        pixels: &[u8],
        width: usize,
        height: usize,
        blend: BlendMode,
    ) {
        let start_x = x - (width / 2) as i32;
        let start_y = y - (height / 2) as i32;
        self.blit_raw(
            start_x - self.camera.x,
            start_y - self.camera.y,
            width,
            height,
            pixels,
            blend,
        );
    }

    fn clip(
        &self,
        start_x: i32,
        start_y: i32,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize, usize, usize)> {
        let row_start = (-start_y).max(0) as usize;
        let row_end = height.min((self.height as i32 - start_y).max(0) as usize);
        let col_start = (-start_x).max(0) as usize;
        let col_end = width.min((self.width as i32 - start_x).max(0) as usize);
        (col_start < col_end && row_start < row_end)
            .then_some((row_start, row_end, col_start, col_end))
    }

    fn blit_raw(
        &mut self,
        start_x: i32,
        start_y: i32,
        width: usize,
        height: usize,
        pixels: &[u8],
        blend: BlendMode,
    ) {
        let Some((row_start, row_end, col_start, col_end)) =
            self.clip(start_x, start_y, width, height)
        else {
            return;
        };
        let frame = self.pixels.frame_mut();
        match blend {
            BlendMode::Opaque => {
                let dst_x = (start_x + col_start as i32) as usize;
                let len = (col_end - col_start) * 4;
                for row in row_start..row_end {
                    let dst_y = (start_y + row as i32) as usize;
                    let src_offset = (row * width + col_start) * 4;
                    let dst_offset = (dst_y * self.width + dst_x) * 4;
                    frame[dst_offset..dst_offset + len]
                        .copy_from_slice(&pixels[src_offset..src_offset + len]);
                }
            }
            BlendMode::Alpha => {
                for row in row_start..row_end {
                    let dst_y = (start_y + row as i32) as usize;
                    for col in col_start..col_end {
                        let dst_x = (start_x + col as i32) as usize;
                        let src_i = (row * width + col) * 4;
                        let dst_i = (dst_y * self.width + dst_x) * 4;
                        let a = pixels[src_i + 3];
                        if a == 255 {
                            frame[dst_i] = pixels[src_i];
                            frame[dst_i + 1] = pixels[src_i + 1];
                            frame[dst_i + 2] = pixels[src_i + 2];
                            frame[dst_i + 3] = 0xff;
                        } else if a > 0 {
                            let a = a as u32;
                            let inv = 255 - a;
                            frame[dst_i] = ((pixels[src_i] as u32 * a + frame[dst_i] as u32 * inv)
                                / 255) as u8;
                            frame[dst_i + 1] = ((pixels[src_i + 1] as u32 * a
                                + frame[dst_i + 1] as u32 * inv)
                                / 255) as u8;
                            frame[dst_i + 2] = ((pixels[src_i + 2] as u32 * a
                                + frame[dst_i + 2] as u32 * inv)
                                / 255) as u8;
                            frame[dst_i + 3] = 0xff;
                        }
                    }
                }
            }
        }
    }

    pub fn set_camera(&mut self, x: i32, y: i32) {
        self.camera.x = x;
        self.camera.y = y;
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
