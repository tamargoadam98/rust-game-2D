use crate::entities::enemy::Enemy;
use crate::entities::player::Player;
use simple_engine::assets::tileset_manager::TilesetManager;
use simple_engine::engine::Game;
use simple_engine::engine::config::Config;
use simple_engine::engine::game_context::GameContext;
use simple_engine::engine::renderer::{Renderable, Renderer};
use simple_engine::entities::entity::Entity;
use simple_engine::world::tilemap::Tilemap;

pub struct MyGame {
    tilemap: Tilemap,
    player: Player,
    enemies: Vec<Enemy>,
}

impl MyGame {
    pub fn new(config: Config) -> Self {
        let mut tileset_manager = TilesetManager::new("assets/sheets/config.json");
        // Fetch background tile map
        let tileset = tileset_manager.tilesets.remove("background").unwrap();
        let tile_size = tileset.tile_size;
        let mut tilemap =
            Tilemap::new(tileset, config.width / tile_size, config.height / tile_size);
        tilemap.fill("stars");

        // Fetch sprite sheet
        let mut spritesheet = tileset_manager.tilesets.remove("ships").unwrap();
        let player_sprite = spritesheet.tiles.remove("player").unwrap();
        let enemy_sprite = spritesheet.tiles.remove("enemy").unwrap();

        // Setup entities
        let player = Player::new(
            (config.width / 2) as f32,
            (config.height / 2) as f32,
            200.0,
            128.0,
            player_sprite,
        );
        let enemies = vec![
            Enemy::new(0.0, 0.0, 100.0, 64.0, enemy_sprite.clone()),
            Enemy::new((config.width - 25) as f32, 0.0, 100.0, 64.0, enemy_sprite),
        ];

        Self {
            tilemap,
            player,
            enemies,
        }
    }
}

impl Game for MyGame {
    fn update(&mut self, ctx: &GameContext) {
        let entity_bounds: Vec<_> = std::iter::once(self.player.get_bounds())
            .chain(self.enemies.iter().map(|e| e.get_bounds()))
            .collect();

        self.player.update(ctx, &entity_bounds);
        for enemy in &mut self.enemies {
            enemy.update(self.player.x, self.player.y, ctx, &entity_bounds);
        }
    }

    fn draw(&self, renderer: &mut Renderer) {
        self.tilemap.draw(renderer);
        self.player.draw(renderer);
        for enemy in &self.enemies {
            enemy.draw(renderer);
        }
    }
}
