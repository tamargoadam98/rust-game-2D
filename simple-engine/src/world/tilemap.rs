use crate::{assets::tileset::Tileset, engine::renderer::{Renderable, Renderer}};

pub struct Tilemap {
    tileset: Tileset,
    tilemap: Vec<Vec<String>>
}

impl Tilemap {
    pub fn new(tileset: Tileset, map_width: usize, map_height: usize) -> Self {
        Self { tileset, tilemap: vec![vec![String::new(); map_width]; map_height] }
    }

    pub fn clear_map(&mut self) {
        for row in &mut self.tilemap {
            for tile in row {
                tile.clear();
            }
        }
    }

    pub fn put_tile(&mut self, tile_id: &str, x: usize, y: usize) {
        self.tilemap[y][x] = tile_id.to_string();
    }

    pub fn fill(&mut self, tile_id: &str) {
        for row in &mut self.tilemap {
            for tile in row {
                *tile = tile_id.to_string();
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &str {
        &self.tilemap[y][x]
    }
}

impl Renderable for Tilemap {
    fn draw(&self, renderer: &mut Renderer) {
        for y in 0 .. self.tilemap.len() {
            for x in 0 .. self.tilemap[0].len() {
                let tile_id = self.get_tile(x, y);
                if !tile_id.is_empty() && let Some(tile) = self.tileset.tiles.get(tile_id) {
                    renderer.blit_tile(
                        x * self.tileset.tile_size, 
                        y * self.tileset.tile_size, 
                        &tile.pixels, 
                        self.tileset.tile_size
                    );
                }
            }
        }
    }
}