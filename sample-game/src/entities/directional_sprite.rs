use std::collections::HashMap;

use simple_engine::assets::tileset::{Tile, Tileset};
use simple_engine::engine::renderer::{BlendMode, Renderer};

use crate::entities::direction::Direction;

#[derive(Clone)]
pub struct DirectionalSprite {
    tileset: Tileset,
    pub direction: Direction,
}

impl DirectionalSprite {
    pub fn new(sprite: Tile, sprite_diag: Tile, size: f32, default_direction: Direction) -> Self {
        let mut tilemap = HashMap::new();
        // Get cardinal direstions
        let mut tile = sprite.scale_to_fit(size as u32);
        tilemap.insert(Direction::Up.to_string(), tile.clone());
        tile = tile.rot_90();
        tilemap.insert(Direction::Right.to_string(), tile.clone());
        tile = tile.rot_90();
        tilemap.insert(Direction::Down.to_string(), tile.clone());
        tile = tile.rot_90();
        tilemap.insert(Direction::Left.to_string(), tile.clone());
        // Get diagonal directions
        tile = sprite_diag.scale_to_fit(size as u32);
        tilemap.insert(Direction::UpRight.to_string(), tile.clone());
        tile = tile.rot_90();
        tilemap.insert(Direction::DownRight.to_string(), tile.clone());
        tile = tile.rot_90();
        tilemap.insert(Direction::DownLeft.to_string(), tile.clone());
        tile = tile.rot_90();
        tilemap.insert(Direction::UpLeft.to_string(), tile.clone());

        Self {
            tileset: Tileset { tiles: tilemap },
            direction: default_direction,
        }
    }

    pub fn update_direction(&mut self, dx: f32, dy: f32) {
        if dx != 0.0 || dy != 0.0 {
            let angle = dy.atan2(dx).to_degrees();
            self.direction = match angle as i32 {
                -22..=22 => Direction::Right,
                23..=67 => Direction::DownRight,
                68..=112 => Direction::Down,
                113..=157 => Direction::DownLeft,
                158..=180 | -180..=-158 => Direction::Left,
                -157..=-113 => Direction::UpLeft,
                -112..=-68 => Direction::Up,
                -67..=-23 => Direction::UpRight,
                _ => self.direction,
            }
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, x: i32, y: i32) {
        let tile = self
            .tileset
            .tiles
            .get(&self.direction.to_string())
            .expect("Missing sprite for direction.");
        renderer.blit_pixels_centered(
            x,
            y,
            &tile.pixels,
            tile.width,
            tile.height,
            BlendMode::Alpha,
        );
    }
}
