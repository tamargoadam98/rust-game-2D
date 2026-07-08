use crate::entities::enemy::Enemy;
use crate::entities::player::Player;
use crate::ui::boost_bar::BoostBar;
use simple_engine::assets::tileset::{Tile, Tileset};
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
    boost_bar: BoostBar,
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
        let tilemap = Self::setup_background_tilemap(config, &mut tileset_manager);

        // Fetch sprite sheet
        let mut spritesheet = tileset_manager.tilesets.remove("ships").unwrap();

        // Setup entities
        let player = Self::setup_player(config, &mut spritesheet);
        let enemies = Self::setup_enemies(config, &mut spritesheet);

        // Setup UI layer
        let boost_bar = Self::setup_ui(&mut tileset_manager);

        Self {
            tilemap,
            player,
            enemies,
            boost_bar,
            camera: Camera::new(300.0),
        }
    }

    fn setup_background_tilemap(config: Config, tileset_manager: &mut TilesetManager) -> Tilemap {
        let tileset = tileset_manager.tilesets.remove("background").unwrap();
        let tile_size = tileset.tile_size;
        let mut tilemap = Tilemap::new(
            tileset,
            config.width.div_ceil(tile_size) + 1,
            config.height.div_ceil(tile_size) + 1,
        );
        tilemap.fill_rand();
        tilemap
    }

    fn setup_player(config: Config, spritesheet: &mut Tileset) -> Player {
        let player_sprite = spritesheet.tiles.remove("player").unwrap();
        let player_sprite_diag = spritesheet.tiles.remove("player_diag").unwrap();
        Player::new(
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
        )
    }

    fn setup_enemies(config: Config, spritesheet: &mut Tileset) -> Vec<Enemy> {
        let enemy_sprite = spritesheet.tiles.remove("enemy").unwrap();
        let enemy_sprite_diag = spritesheet.tiles.remove("enemy_diag").unwrap();
        let enemy_config = ActorConfig {
            max_speed: 400.0,
            acceleration: 200.0,
            deceleration: 1.0,
            box_size: 96.0,
        };
        vec![
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
        ]
    }

    fn setup_ui(tileset_manager: &mut TilesetManager) -> BoostBar {
        let mut tileset = tileset_manager.tilesets.remove("ui_elements").unwrap();
        let boost_bar_tiles: [Tile; 8] =
            std::array::from_fn(|i| tileset.tiles.remove(&format!("boost_bar_{i}")).unwrap());
        let fuel_tiles: [Tile; 8] =
            std::array::from_fn(|i| tileset.tiles.remove(&format!("fuel_bar_{i}")).unwrap());
        BoostBar::new(20, 20, boost_bar_tiles, fuel_tiles, 64)
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
        self.boost_bar
            .set_boost_percentage(self.player.get_boost_percentage());
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

    fn draw_ui(&self, renderer: &mut Renderer) {
        self.boost_bar.draw(renderer);
    }
}
