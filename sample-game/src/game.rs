use crate::entities::enemy::Enemy;
use crate::entities::player::Player;
use simple_engine::assets::tileset_manager::TilesetManager;
use simple_engine::engine::Game;
use simple_engine::engine::camera::Camera;
use simple_engine::engine::config::Config;
use simple_engine::engine::game_context::GameContext;
use simple_engine::engine::renderer::{Renderable, Renderer};
use simple_engine::entities::actor::ActorConfig;
use simple_engine::entities::entity::Entity;
use simple_engine::world::tilemap::Tilemap;

pub struct MyGame {
    tilemap: Tilemap,
    player: Player,
    enemies: Vec<Enemy>,
    camera: Camera,
}

impl MyGame {
    pub fn new(config: Config) -> Self {
        let max_background_tiles_across = 16;
        let mut tileset_manager = TilesetManager::new(
            "assets/sheets/config.json",
            (config.width / max_background_tiles_across).next_multiple_of(64),
        );
        // Fetch background tile map
        let tileset = tileset_manager.tilesets.remove("background").unwrap();
        let tile_size = tileset.tile_size;
        let mut tilemap = Tilemap::new(
            tileset,
            config.width / tile_size + 1,
            config.height / tile_size + 1,
        );
        tilemap.fill_rand();

        // Fetch sprite sheet
        let mut spritesheet = tileset_manager.tilesets.remove("ships").unwrap();
        let player_sprite = spritesheet.tiles.remove("player").unwrap();
        let player_sprite_diag = spritesheet.tiles.remove("player_diag").unwrap();
        let enemy_sprite = spritesheet.tiles.remove("enemy").unwrap();
        let enemy_sprite_diag = spritesheet.tiles.remove("enemy_diag").unwrap();

        // Setup entities
        let player = Player::new(
            (config.width / 2) as f32,
            (config.height / 2) as f32,
            ActorConfig {
                max_speed: 500.0,
                acceleration: 800.0,
                deceleration: 8.0,
                box_size: 128.0,
            },
            player_sprite,
            player_sprite_diag,
        );
        let enemy_config = ActorConfig {
            max_speed: 400.0,
            acceleration: 200.0,
            deceleration: 1.0,
            box_size: 96.0,
        };
        let enemies = vec![
            Enemy::new(
                0.0,
                0.0,
                enemy_config,
                enemy_sprite.clone(),
                enemy_sprite_diag.clone(),
            ),
            Enemy::new(
                (config.width - 25) as f32,
                0.0,
                enemy_config,
                enemy_sprite,
                enemy_sprite_diag,
            ),
        ];

        Self {
            tilemap,
            player,
            enemies,
            camera: Camera::new(300.0),
        }
    }

    fn update_camera(&mut self, ctx: &GameContext) {
        self.camera.follow(
            self.player.x(),
            self.player.y(),
            ctx.config.width as f32,
            ctx.config.height as f32,
        );
    }
}

impl Game for MyGame {
    fn update(&mut self, ctx: &GameContext) {
        let entity_bounds: Vec<_> = std::iter::once(self.player.bounds())
            .chain(self.enemies.iter().map(|e| e.bounds()))
            .collect();

        self.player.update(ctx, &entity_bounds);
        for enemy in &mut self.enemies {
            enemy.update(self.player.x(), self.player.y(), ctx, &entity_bounds);
        }
        self.update_camera(ctx);
    }

    fn draw(&self, renderer: &mut Renderer) {
        renderer.set_camera(self.camera.x, self.camera.y);
        self.tilemap.draw(renderer);
        self.player.draw(renderer);
        for enemy in &self.enemies {
            enemy.draw(renderer);
        }
    }
}
