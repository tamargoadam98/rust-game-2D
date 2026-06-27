use std::collections::HashMap;

use image::imageops::FilterType;

#[derive(Clone)]
pub struct Tile {
    pub pixels: Vec<u8>, // raw RGBA bytes, 4 bytes per pixel
    pub size: usize,
}

impl Tile {
    pub fn scale(&self, new_size: u32) -> Tile {
        let img =
            image::RgbaImage::from_raw(self.size as u32, self.size as u32, self.pixels.clone())
                .expect("pixel buffer size doesn't match tile dimensions");
        let scaled = image::imageops::resize(&img, new_size, new_size, FilterType::Nearest);
        Tile {
            pixels: scaled.into_raw(),
            size: new_size as usize,
        }
    }
}

pub struct Tileset {
    pub tile_size: usize,
    pub tiles: HashMap<String, Tile>,
}
