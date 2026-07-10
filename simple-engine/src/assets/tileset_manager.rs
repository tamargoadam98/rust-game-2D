use std::collections::HashMap;

use image::ImageReader;

use crate::assets::tileset::{Tile, Tileset};
use crate::assets::tileset_config::{TileConfig, TilesetConfig};

pub struct TilesetManager {
    pub tilesets: HashMap<String, Tileset>,
}

impl TilesetManager {
    pub fn new(config_path: &str) -> Self {
        Self {
            tilesets: Self::load_tilesets(config_path),
        }
    }

    fn load_tilesets(config_path: &str) -> HashMap<String, Tileset> {
        let config_str = std::fs::read_to_string(config_path).expect("Failed to read config.json");
        let tileset_configs: Vec<TilesetConfig> =
            serde_json::from_str(&config_str).expect("Failed to parse config.json");
        let mut tilesets = HashMap::<String, Tileset>::new();

        for tileset_config in tileset_configs {
            let tiles = Self::extract_tiles(&tileset_config.file, &tileset_config.tile_configs, tileset_config.scale);
            tilesets.insert(tileset_config.id, Tileset { tiles });
        }
        tilesets
    }

    fn extract_tiles(
        file: &str,
        tile_configs: &[TileConfig],
        scale: Option<u32>,
    ) -> HashMap<String, Tile> {
        let img = ImageReader::open(file)
            .expect("Failed to open image file.")
            .decode()
            .expect("Failed to decode image file.");

        let rgba = img.to_rgba8();
        let image_width = rgba.width() as usize;
        let rgba_bytes = rgba.into_raw();

        let mut tiles = HashMap::new();

        for tile_config in tile_configs {
            let pixels = Self::extract_tile(
                &rgba_bytes,
                image_width,
                tile_config.x,
                tile_config.y,
                tile_config.width,
                tile_config.height,
            );
            let tile = Tile {
                pixels,
                width: tile_config.width,
                height: tile_config.height,
            };
            let scaled = match scale {
                Some(s) => tile.scale_to_fit(s),
                None => tile,
            };
            tiles.insert(tile_config.name.clone(), scaled);
        }
        tiles
    }

    /// Extracts a rectangle of pixels at pixel coordinates `(x, y)` with size `(width, height)`.
    fn extract_tile(
        rgba_bytes: &[u8],
        file_width: usize,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Vec<u8> {
        let mut pixels = Vec::with_capacity(width * height * 4);
        for row in 0..height {
            let pixel_y = y + row;
            let start = (pixel_y * file_width + x) * 4;
            pixels.extend_from_slice(&rgba_bytes[start..start + width * 4]);
        }
        pixels
    }
}
