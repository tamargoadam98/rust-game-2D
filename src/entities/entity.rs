use crate::engine::game_context::GameContext;
use crate::engine::renderer::Renderable;

pub trait Entity: Renderable {
    fn update(&mut self, ctx: &GameContext);
}
