use std::collections::HashMap;

use image::{
    ImageBuffer, Rgba,
    imageops::{self, FilterType},
};

#[derive(Clone)]
pub struct Tile {
    pub pixels: Vec<u8>, // raw RGBA bytes, 4 bytes per pixel
    pub size: usize,
}

impl Tile {
    pub fn scale(&self, new_size: u32) -> Tile {
        let img = self.to_image();
        let scaled = image::imageops::resize(&img, new_size, new_size, FilterType::Nearest);
        Tile {
            pixels: scaled.into_raw(),
            size: new_size as usize,
        }
    }

    pub fn rot_90(&self) -> Tile {
        let img = self.to_image();
        let rotated = imageops::rotate90(&img);
        Tile {
            pixels: rotated.into_raw(),
            size: self.size,
        }
    }

    fn to_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        image::RgbaImage::from_raw(self.size as u32, self.size as u32, self.pixels.clone())
            .expect("pixel buffer size doesn't match tile dimensions")
    }
}

pub struct Tileset {
    pub tile_size: usize,
    pub tiles: HashMap<String, Tile>,
}
