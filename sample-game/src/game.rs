use crate::entities::enemy::Enemy;
use crate::entities::laser::Laser;
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
    lasers: Vec<Laser>,
    boost_bar: BoostBar,
    camera: Camera,
}

impl MyGame {
    pub fn new(config: Config) -> Self {
        let mut tileset_manager = TilesetManager::new("assets/sheets/config.json");

        // Fetch background tile map
        let tilemap = Self::setup_background_tilemap(config, &mut tileset_manager);

        // Fetch sprite sheet
        let mut spritesheet = tileset_manager.tilesets.remove("ships").unwrap();

        // Setup entities
        let player = Self::setup_player(config, &mut spritesheet);
        let enemies = Self::setup_enemies(config, &mut spritesheet);
        let lasers: Vec<Laser> = Vec::new();

        // Setup UI layer
        let boost_bar = Self::setup_ui(config, &mut tileset_manager);

        Self {
            tilemap,
            player,
            enemies,
            lasers,
            boost_bar,
            camera: Camera::new(300.0),
        }
    }

    fn setup_background_tilemap(config: Config, tileset_manager: &mut TilesetManager) -> Tilemap {
        let tileset = tileset_manager.tilesets.remove("background").unwrap();
        let tile_size = tileset.tiles.values().next().map(|t| t.width).unwrap_or(64);
        let mut tilemap = Tilemap::new(
            tileset,
            tile_size,
            config.width.div_ceil(tile_size) + 1,
            config.height.div_ceil(tile_size) + 1,
        );
        tilemap.fill_rand();
        tilemap
    }

    fn setup_player(config: Config, spritesheet: &mut Tileset) -> Player {
        let player_sprite = spritesheet.tiles.remove("player").unwrap();
        let player_sprite_diag = spritesheet.tiles.remove("player_diag").unwrap();
        let laser_sprite = spritesheet.tiles.remove("green_laser").unwrap();
        let laser_sprite_diag = spritesheet.tiles.remove("green_laser_diag").unwrap();
        Player::new(
            (config.width / 2) as f32,
            (config.height / 2) as f32,
            ActorConfig {
                max_speed: 500.0,
                acceleration: 800.0,
                deceleration: 25.0,
                box_size: 128.0,
            },
            player_sprite,
            player_sprite_diag,
            laser_sprite,
            laser_sprite_diag,
        )
    }

    fn setup_enemies(config: Config, spritesheet: &mut Tileset) -> Vec<Enemy> {
        let enemy_sprite = spritesheet.tiles.remove("enemy").unwrap();
        let enemy_sprite_diag = spritesheet.tiles.remove("enemy_diag").unwrap();
        let enemy_config = ActorConfig {
            max_speed: 400.0,
            acceleration: 200.0,
            deceleration: 10.0,
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

    fn setup_ui(config: Config, tileset_manager: &mut TilesetManager) -> BoostBar {
        let mut tileset = tileset_manager.tilesets.remove("ui_elements").unwrap();
        let boost_bar_tiles: [Tile; 8] =
            std::array::from_fn(|i| tileset.tiles.remove(&format!("boost_bar_{i}")).unwrap());
        let fuel_tiles: [Tile; 8] =
            std::array::from_fn(|i| tileset.tiles.remove(&format!("fuel_bar_{i}")).unwrap());
        let tile_size = boost_bar_tiles[0].height as u32;
        BoostBar::new(
            32,
            (config.height as u32 - tile_size - 32).try_into().unwrap(),
            boost_bar_tiles,
            fuel_tiles,
        )
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

        let laser_option = self.player.update(ctx, &entity_bounds);
        self.lasers
            .retain_mut(|laser| laser.update(ctx, &entity_bounds));
        if let Some(laser) = laser_option {
            self.lasers.push(laser);
        }

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
        for laser in &self.lasers {
            laser.draw(renderer);
        }
        self.player.draw(renderer);
        for enemy in &self.enemies {
            enemy.draw(renderer);
        }
    }

    fn draw_ui(&self, renderer: &mut Renderer) {
        self.boost_bar.draw(renderer);
    }
}
