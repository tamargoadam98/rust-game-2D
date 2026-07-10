use std::collections::HashMap;

use image::{
    ImageBuffer, Rgba,
    imageops::{self, FilterType},
};

#[derive(Clone)]
pub struct Tile {
    pub pixels: Vec<u8>, // raw RGBA bytes, 4 bytes per pixel
    pub width: usize,
    pub height: usize,
}

impl Tile {
    /// Scales so the longest dimension equals `size`, preserving aspect ratio.
    pub fn scale_to_fit(&self, size: u32) -> Tile {
        let longest = self.width.max(self.height) as f32;
        let w = (size as f32 * self.width as f32 / longest).round() as u32;
        let h = (size as f32 * self.height as f32 / longest).round() as u32;
        self.scale(w, h)
    }

    pub fn scale(&self, new_width: u32, new_height: u32) -> Tile {
        let img = self.to_image();
        let scaled = image::imageops::resize(&img, new_width, new_height, FilterType::Nearest);
        Tile {
            pixels: scaled.into_raw(),
            width: new_width as usize,
            height: new_height as usize,
        }
    }

    pub fn rot_90(&self) -> Tile {
        let img = self.to_image();
        let rotated = imageops::rotate90(&img);
        Tile {
            pixels: rotated.into_raw(),
            width: self.height,
            height: self.width,
        }
    }

    fn to_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        image::RgbaImage::from_raw(self.width as u32, self.height as u32, self.pixels.clone())
            .expect("pixel buffer size doesn't match tile dimensions")
    }
}

#[derive(Clone)]
pub struct Tileset {
    pub tiles: HashMap<String, Tile>,
}
