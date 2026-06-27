use std::collections::HashMap;

use image::ImageReader;

use crate::assets::tileset::{Tile, Tileset};
use crate::assets::tileset_config::{TileConfig, TilesetConfig};

pub struct TilesetManager {
    pub tilesets: HashMap<String, Tileset>,
}

impl TilesetManager {
    /// Loads tilesets from the JSON config at `config_path` relative to the working directory.
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
            let tilemap = Self::extract_tilemap(
                &tileset_config.file,
                &tileset_config.tile_configs,
                tileset_config.tile_size,
            );
            tilesets.insert(
                tileset_config.id,
                Tileset {
                    tile_size: tileset_config.tile_size,
                    tiles: tilemap,
                },
            );
        }
        tilesets
    }

    /// Loads a PNG sheet and extracts all named tiles from it into a lookup map.
    fn extract_tilemap(
        file: &str,
        tile_configs: &[TileConfig],
        tile_size: usize,
    ) -> HashMap<String, Tile> {
        let img = ImageReader::open(file)
            .expect("Failed to open image file.")
            .decode()
            .expect("Failed to decode image file.");

        let rgba = img.to_rgba8();
        let image_width = rgba.width() as usize;
        let rgba_bytes = rgba.into_raw(); // raw RGBA bytes, 4 per pixel

        let mut tilemap = HashMap::new();

        for tile_config in tile_configs {
            let pixels = Self::extract_tile(
                &rgba_bytes,
                image_width,
                tile_config.x,
                tile_config.y,
                tile_size,
            );
            tilemap.insert(tile_config.name.clone(), Tile { pixels, size: tile_size });
        }
        tilemap
    }

    /// Slices a tile at tile coordinates `(tx, ty)` out of a flat row-major RGBA byte buffer.
    fn extract_tile(
        rgba_bytes: &[u8],
        file_width: usize,
        tx: usize,
        ty: usize,
        tile_size: usize,
    ) -> Vec<u8> {
        let mut pixels = Vec::with_capacity(tile_size * tile_size * 4);
        for row in 0..tile_size {
            let pixel_y = ty * tile_size + row;
            let start = (pixel_y * file_width + tx * tile_size) * 4;
            pixels.extend_from_slice(&rgba_bytes[start..start + tile_size * 4]);
        }
        pixels
    }
}
