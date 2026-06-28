use crate::{assets::tileset::Tile, entities::bounds::Bounds};

pub struct Actor {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub box_size: f32,
    pub sprite: Tile,
}

impl Actor {
    pub fn bounds(&self) -> Bounds {
        Bounds::new(self.id, self.x, self.y, self.box_size, self.box_size)
    }
}
