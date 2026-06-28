use std::sync::atomic::{AtomicU32, Ordering};

use crate::engine::renderer::Renderable;
use crate::entities::bounds::Bounds;

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

/// A game object that can be rendered and has spatial bounds for collision detection.
pub trait Entity: Renderable {
    /// Returns a unique ID for this entity. Call once in `new()` and store the result.
    fn next_id() -> u32
    where
        Self: Sized,
    {
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    }

    fn bounds(&self) -> Bounds;
}
