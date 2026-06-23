use crate::{game_context::GameContext, renderer::Renderable};

pub trait Entity: Renderable {
    fn update(&mut self, ctx: &GameContext);
}
