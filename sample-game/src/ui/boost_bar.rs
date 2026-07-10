use simple_engine::{
    assets::tileset::Tile,
    engine::renderer::{BlendMode, Renderable, Renderer},
};

pub struct BoostBar {
    x: i32,
    y: i32,
    boost_bar_tiles: [Tile; 8],
    fuel_tiles: [Tile; 8],
    boost_percentage: f32,
}

impl BoostBar {
    pub fn new(x: i32, y: i32, bar_tiles: [Tile; 8], fuel_tiles: [Tile; 8]) -> Self {
        Self {
            x,
            y,
            boost_bar_tiles: bar_tiles,
            fuel_tiles,
            boost_percentage: 1.0,
        }
    }

    pub fn set_boost_percentage(&mut self, boost_percentage: f32) {
        self.boost_percentage = boost_percentage.clamp(0.0, 1.0);
    }
}

impl Renderable for BoostBar {
    fn draw(&self, renderer: &mut Renderer) {
        for (i, tile) in self.boost_bar_tiles.iter().enumerate() {
            renderer.blit_pixels(
                self.x + (tile.width * i) as i32,
                self.y,
                &tile.pixels,
                tile.width,
                tile.height,
                BlendMode::Alpha,
            )
        }
        let fuel_bar_len = (self.fuel_tiles.len() as f32 * self.boost_percentage).round() as usize;
        for i in 0..fuel_bar_len {
            let tile = &self.fuel_tiles[i];
            renderer.blit_pixels(
                self.x + (tile.width * i) as i32,
                self.y,
                &tile.pixels,
                tile.width,
                tile.height,
                BlendMode::Alpha,
            )
        }
    }
}
