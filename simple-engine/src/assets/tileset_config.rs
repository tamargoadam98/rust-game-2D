use serde::Deserialize;

#[derive(Deserialize)]
pub struct TileConfig {
    pub name: String,
    pub x: usize,      // pixel x in source image
    pub y: usize,      // pixel y in source image
    pub width: usize,  // pixel width in source image
    pub height: usize, // pixel height in source image
}

#[derive(Deserialize)]
pub struct TilesetConfig {
    pub id: String,
    pub file: String,
    pub scale: Option<u32>, // display size (longest dimension); if None, source resolution is used
    pub tile_configs: Vec<TileConfig>,
}
