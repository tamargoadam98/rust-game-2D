use std::sync::atomic::{AtomicU32, Ordering};

use crate::engine::game_context::GameContext;
use crate::engine::renderer::Renderable;
use crate::entities::bounds::Bounds;

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

pub trait Entity: Renderable {
    fn next_id() -> u32 where Self: Sized {
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    }
    
    fn update(&mut self, ctx: &GameContext);

    fn get_bounds(&self) -> Bounds;
}
