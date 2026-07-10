use crate::{
    assets::tileset::Tileset,
    engine::renderer::{BlendMode, Renderable, Renderer},
};

pub struct Tilemap {
    tileset: Tileset,
    tilemap: Vec<Vec<String>>,
    tile_size: usize,
}

impl Tilemap {
    pub fn new(tileset: Tileset, tile_size: usize, map_width: usize, map_height: usize) -> Self {
        Self {
            tileset,
            tilemap: vec![vec![String::new(); map_width]; map_height],
            tile_size,
        }
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

    pub fn fill_rand(&mut self) {
        use rand::seq::IndexedRandom;
        let keys: Vec<String> = self.tileset.tiles.keys().cloned().collect();
        let mut rng = rand::rng();
        for row in &mut self.tilemap {
            for tile in row {
                if let Some(id) = keys.choose(&mut rng) {
                    *tile = id.clone();
                }
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &str {
        &self.tilemap[y][x]
    }

    fn scroll_start_and_offset(camera_pos: i32, tile_size: i32, map_len: i32) -> (usize, i32) {
        let start = camera_pos.div_euclid(tile_size).rem_euclid(map_len) as usize;
        let offset = camera_pos.rem_euclid(tile_size);
        (start, offset)
    }
}

impl Renderable for Tilemap {
    fn draw(&self, renderer: &mut Renderer) {
        let tile_size = self.tile_size;
        let (x_start, off_x) = Self::scroll_start_and_offset(
            renderer.camera.x,
            tile_size as i32,
            self.tilemap[0].len() as i32,
        );
        let (y_start, off_y) = Self::scroll_start_and_offset(
            renderer.camera.y,
            tile_size as i32,
            self.tilemap.len() as i32,
        );

        for y in 0..self.tilemap.len() {
            let y_mod = (y_start + y) % self.tilemap.len();
            for x in 0..self.tilemap[0].len() {
                let x_mod = (x_start + x) % self.tilemap[0].len();
                let tile_id = self.get_tile(x_mod, y_mod);
                if !tile_id.is_empty()
                    && let Some(tile) = self.tileset.tiles.get(tile_id)
                {
                    renderer.blit_pixels(
                        (x * tile_size) as i32 - off_x,
                        (y * tile_size) as i32 - off_y,
                        &tile.pixels,
                        tile_size,
                        tile_size,
                        BlendMode::Opaque,
                    );
                }
            }
        }
    }
}
