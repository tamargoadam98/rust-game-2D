use std::collections::HashMap;

pub struct Tile {
    pub pixels: Vec<u8>, // raw RGBA bytes, 4 bytes per pixel
}

pub struct Tileset {
    pub tile_size: usize,
    pub tiles: HashMap<String, Tile>,
}
