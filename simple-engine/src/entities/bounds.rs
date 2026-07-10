/// Axis-aligned bounding box used for collision detection. Edges are computed from a center position.
pub struct Bounds {
    pub id: u32,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Bounds {
    pub fn new(id: u32, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            id,
            left: x - width / 2.0,
            right: x + width / 2.0,
            top: y - height / 2.0,
            bottom: y + height / 2.0,
        }
    }

    /// Returns true if this overlaps `bounds`. Entities with the same ID never collide.
    pub fn is_collision(&self, bounds: &Bounds) -> bool {
        self.id != bounds.id
            && self.left < bounds.right
            && self.right > bounds.left
            && self.top < bounds.bottom
            && self.bottom > bounds.top
    }

    pub fn check_collisions(&self, bounds_vec: &[Bounds]) -> bool {
        bounds_vec.iter().any(|b| self.is_collision(b))
    }

    /// Like `check_collisions`, but skips the entry with `exclude_id`.
    pub fn check_collisions_excluding(&self, bounds_vec: &[Bounds], exclude_id: u32) -> bool {
        bounds_vec.iter().any(|b| b.id != exclude_id && self.is_collision(b))
    }
}
