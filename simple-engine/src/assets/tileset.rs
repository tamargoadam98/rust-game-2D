use std::collections::HashMap;

pub struct Tile {
    pub pixels: Vec<u32>,
}

pub struct Tileset {
    pub tile_size: usize,
    pub tiles: HashMap<String, Tile>,
}
