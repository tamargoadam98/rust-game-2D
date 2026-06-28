use std::collections::HashMap;

use simple_engine::assets::tileset::{Tile, Tileset};
use simple_engine::engine::renderer::Renderer;

use crate::entities::direction::Direction;

pub struct DirectionalSprite {
    tileset: Tileset,
    pub direction: Direction,
}

impl DirectionalSprite {
    pub fn new(sprite: Tile, size: f32, default_direction: Direction) -> Self {
        let mut tile = sprite.scale(size as u32);
        let mut tilemap = HashMap::new();
        tilemap.insert(Direction::Up.to_string(), tile.clone());
        tile = tile.rot_90();
        tilemap.insert(Direction::Right.to_string(), tile.clone());
        tile = tile.rot_90();
        tilemap.insert(Direction::Down.to_string(), tile.clone());
        tile = tile.rot_90();
        tilemap.insert(Direction::Left.to_string(), tile.clone());
        Self {
            tileset: Tileset {
                tile_size: size as usize,
                tiles: tilemap,
            },
            direction: default_direction,
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, x: usize, y: usize) {
        let tile = self
            .tileset
            .tiles
            .get(&self.direction.to_string())
            .expect("Missing sprite for direction.");
        renderer.blit_pixels_centered(
            x,
            y,
            &tile.pixels,
            self.tileset.tile_size,
            self.tileset.tile_size,
        );
    }
}
