use crate::enemies::{Enemy, EnemySystem};
use crate::misc::overlaps;
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::mint::{Point2, Vector2};
use ggez::{graphics, Context, GameResult};
use std::time::Duration;

static PROJECTILE_BASE_SPEED: f32 = 0.7;
static PLAYER_SHOT_VELOCITY: Vector2<f32> = Vector2 {
    x: 0.0,
    y: -PROJECTILE_BASE_SPEED,
};

pub enum ShotType {
    PlayerShot,
    EnemyShot,
}

static PLAYER_SHOT_IMG: &str = "/images/player_shot.png";

pub struct Projectile {
    image: Image,
    transform: Vector2<f32>,
    velocity: Vector2<f32>,
}

impl Projectile {
    pub fn new(
        ctx: &mut Context,
        shot_type: ShotType,
        transform: Vector2<f32>,
        velocity: Vector2<f32>,
    ) -> Projectile {
        let img_path = match shot_type {
            ShotType::PlayerShot => PLAYER_SHOT_IMG,
            ShotType::EnemyShot => "",
        };

        Projectile {
            image: Image::new(ctx, img_path).unwrap(),
            transform,
            velocity,
        }
    }

    pub fn update(&mut self, delta_t: f32) {
        self.transform.x = self.transform.x + self.velocity.x * delta_t;
        self.transform.y = self.transform.y + self.velocity.y * delta_t;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.image, DrawParam::new().dest(self.transform))
    }

    pub fn bb(&self) -> Rect {
        let dimensions = self.image.dimensions();
        let position = self.transform;

        return Rect {
            x: position.x,
            y: position.y,
            w: dimensions.w,
            h: dimensions.h,
        };
    }

    pub fn off_screen(&self) -> bool {
        // we know projectiles only move one way, that is, up
        // when the projectile has gone beyond the screen, up to its own
        // height, this is true
        return self.transform.y < (0.0 - self.image.dimensions().h as f32);
    }
}

pub struct ProjectileSystem {
    projectiles: Vec<Option<Projectile>>,
}

impl ProjectileSystem {
    pub fn new() -> ProjectileSystem {
        ProjectileSystem {
            projectiles: vec![],
        }
    }

    pub fn add_projectile(
        &mut self,
        ctx: &mut Context,
        shot_type: ShotType,
        shot_from: Vector2<f32>,
    ) {
        let mut empty_slot: isize = -1;

        // let's iterate through the vector and see if we can find
        // an empty slot for the projectile, if we do, we'll capture
        // index
        for (idx, projectile_slot) in self.projectiles.iter().enumerate() {
            if projectile_slot.is_none() {
                empty_slot = idx as isize;
                break;
            }
        }

        let projectile = Some(Projectile::new(
            ctx,
            shot_type,
            shot_from,
            PLAYER_SHOT_VELOCITY,
        ));

        if empty_slot >= 0 {
            self.projectiles[empty_slot as usize] = projectile;
        } else {
            self.projectiles.push(projectile);
        }
    }

    pub fn update(&mut self, enemies: &mut EnemySystem, dt: f32) {
        // update projectile movement and check collision with enemy
        // or if the projectile has gone off-screen
        for projectile_wrapped in &mut self.projectiles {
            match projectile_wrapped {
                Some(projectile) => {
                    projectile.update(dt);

                    if projectile.off_screen() {
                        *projectile_wrapped = None;
                        break;
                    }

                    match enemies.collides_with(&projectile.bb()) {
                        Some(enemy_id) => {
                            enemies.clear_enemy(enemy_id);
                            *projectile_wrapped = None;
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        for projectile in self.projectiles.iter() {
            match projectile {
                Some(projectile) => projectile.draw(ctx)?,
                _ => {}
            }
        }

        Ok(())
    }
}
