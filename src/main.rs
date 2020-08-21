mod enemies;
mod image_cache;
mod misc;
mod player;
mod projectile;
mod timed_tracker;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use rand::{thread_rng, Rng};

use ggez::conf::WindowSetup;
use ggez::event::EventHandler;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{
    Color, DrawParam, Image, Mesh, Rect, Scale, Text, TextFragment, BLACK, WHITE,
};
use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::mint::Vector2;
use ggez::*;

use crate::enemies::{Enemy, EnemySystem};
use crate::image_cache::ImageCache;
use crate::player::{Direction, Player};
use crate::projectile::{Projectile, ProjectileSystem};
use ggez::graphics::mint::Point2;

type ScreenBounds = (f32, f32);
type BoundingBox = Rect;
enum GAME_STATE {
    PLAYING,
    OVER,
}

static SCREEN_BOUNDS: ScreenBounds = (800.0, 600.0);

fn main() {
    let resource_dir = PathBuf::from("./assets");

    let mut c = conf::Conf::new();
    c.window_setup = WindowSetup::default().title("ggez-space-invaders");

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("ggez-space-invaders", "shak")
        .conf(c)
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let game_state = &mut Game::new(ctx, SCREEN_BOUNDS);

    event::run(ctx, event_loop, game_state).unwrap();
}

struct Game {
    enemies: EnemySystem,
    player: Player,
    projectiles: ProjectileSystem,
    key_presses: HashMap<KeyCode, Instant>,
    debugging: bool,
    image_cache: ImageCache,
    state: GAME_STATE,
    game_over_text: Text,
}

impl Game {
    fn new(ctx: &mut Context, screen_bounds: ScreenBounds) -> Game {
        let mut player = Player::new(ctx, "/images/player.png");
        player.move_to(screen_bounds.0 / 2.0, screen_bounds.1 - 100.0);

        let mut image_cache = ImageCache::new();

        image_cache.image(ctx, "/images/enemy_explode.png");

        let mut game = Game {
            enemies: EnemySystem::new(ctx, &mut image_cache),
            player,
            projectiles: ProjectileSystem::new(),
            key_presses: HashMap::new(),
            debugging: false,
            image_cache,
            state: GAME_STATE::PLAYING,
            game_over_text: Text::new(TextFragment {
                text: "GAME OVER".to_string(),
                color: Some(WHITE),
                font: None,
                scale: Some(Scale::uniform(100.0)),
            }),
        };

        game
    }

    fn update_playing(&mut self, ctx: &mut Context, dt: f32) -> GameResult {
        if is_key_pressed(ctx, KeyCode::Space) {
            self.player.shoot(ctx, &mut self.projectiles);
        }

        let collided = self.enemies.update(&self.player.bb(), SCREEN_BOUNDS);

        if collided {
            self.state = GAME_STATE::OVER;
        }

        process_player_movement(ctx, &mut self.player);
        self.projectiles.update(&mut self.enemies, dt);

        Ok(())
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = timer::delta(ctx).as_millis() as f32;

        if is_key_pressed(ctx, KeyCode::P) {
            self.debugging = !self.debugging;
        }

        if is_key_pressed(ctx, KeyCode::O) {
            self.state = GAME_STATE::OVER;
        }

        match self.state {
            GAME_STATE::PLAYING => {
                self.update_playing(ctx, dt);
            }
            GAME_STATE::OVER => {}
        }

        if self.debugging {
            println!("entering debug pause");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);

        match self.state {
            GAME_STATE::PLAYING => {
                self.enemies.draw(ctx)?;
                self.projectiles.draw(ctx)?;
                self.player.draw(ctx)?;
            }
            GAME_STATE::OVER => {
                let game_over_text_dimensions = self.game_over_text.dimensions(ctx);

                graphics::draw(
                    ctx,
                    &self.game_over_text,
                    (
                        Point2 {
                            x: (SCREEN_BOUNDS.0 - game_over_text_dimensions.0 as f32) / 2.0,
                            y: (SCREEN_BOUNDS.1 - game_over_text_dimensions.1 as f32) / 2.0,
                        },
                        graphics::WHITE,
                    ),
                )?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn process_player_movement(ctx: &mut Context, player: &mut Player) {
    let left_bound = 0.0;
    let right_bound = SCREEN_BOUNDS.0 - player.image.dimensions().w;

    if is_key_pressed(ctx, KeyCode::Left) && player.transform.x > left_bound {
        player.move_player(Direction::Left);
    } else if is_key_pressed(ctx, KeyCode::Right) && player.transform.x < right_bound {
        player.move_player(Direction::Right);
    }

    if is_key_pressed(ctx, KeyCode::LShift) {
        player.boost();
    }
}
