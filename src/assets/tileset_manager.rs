use std::collections::HashMap;

use image::ImageReader;

use crate::assets::tileset_config::{TileConfig, TilesetConfig};
use crate::assets::tileset::{Tile, Tileset};


/// Loads tilesets from `assets/sheets/config.json` relative to the working directory.
pub struct TilesetManager {
    pub tilesets: HashMap<String, Tileset>
}

impl TilesetManager {
    pub fn new() -> Self {
        Self { tilesets: Self::load_tilesets() }
    }

    fn load_tilesets() -> HashMap<String, Tileset> {
        let config_path = "assets/sheets/config.json";
        let config_str = std::fs::read_to_string(config_path).expect("Failed to read config.json");
        let tileset_configs: Vec<TilesetConfig> = serde_json::from_str(&config_str).expect("Failed to parse config.json");
        let mut tilesets = HashMap::<String, Tileset>::new();

        for tileset_config in tileset_configs {
            let tilemap = Self::extract_tilemap(
                &tileset_config.file,
                &tileset_config.tile_configs,
                tileset_config.tile_size
            );
            tilesets.insert(tileset_config.id, Tileset{ tile_size: tileset_config.tile_size, tiles: tilemap });
        }
        tilesets
    }

    /// Loads a PNG sheet and extracts all named tiles from it into a lookup map.
    fn extract_tilemap(file: &str, tile_configs: &[TileConfig], tile_size: usize) -> HashMap<String, Tile> {
        let img = ImageReader::open(file)
            .expect("Failed to open image file.")
            .decode()
            .expect("Failed to decode image file.");

        let rgba = img.to_rgba8();
        let image_width = rgba.width() as usize;

        let rgb: Vec<u32> = rgba.pixels()
            .map(|pixel| {
                let r = pixel[0] as u32;
                let g = pixel[1] as u32;
                let b = pixel[2] as u32;
                (r << 16) | (g << 8) | b // Pack into 0xRRGGBB (ignoring alpha)
            })
            .collect();

        let mut tilemap = HashMap::new();

        for tile_config in tile_configs {
            let pixels = Self::extract_tile(&rgb, image_width, tile_config.x, tile_config.y, tile_size);
            tilemap.insert(tile_config.name.clone(), Tile { pixels });
        }
        tilemap
    }

    /// Slices a tile at tile coordinates `(tx, ty)` out of a flat row-major pixel buffer.
    fn extract_tile(rgb: &[u32], file_width: usize, tx: usize, ty: usize, tile_size: usize) -> Vec<u32> {
        let mut pixels = Vec::with_capacity(tile_size * tile_size);
        for row in 0..tile_size {
            let pixel_y = ty * tile_size + row;
            let start = pixel_y * file_width + tx * tile_size;
            pixels.extend_from_slice(&rgb[start..start + tile_size]);
        }
        pixels
    }
}