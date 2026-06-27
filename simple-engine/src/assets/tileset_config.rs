use serde::Deserialize;

#[derive(Deserialize)]
pub struct TileConfig {
    pub name: String,
    pub x: usize,
    pub y: usize,
}

#[derive(Deserialize)]
pub struct TilesetConfig {
    pub id: String,
    pub file: String,
    pub tile_size: usize,
    pub tile_configs: Vec<TileConfig>,
}
