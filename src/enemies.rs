use crate::image_cache::ImageCache;
use crate::misc::overlaps;
use crate::player::Player;
use crate::timed_tracker::TimedTracker;
use crate::{BoundingBox, ScreenBounds};
use ggez::graphics::mint::Point2;
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::{graphics, Context, GameResult};
use rand::Rng;
use std::time::Duration;

static ENEMY_LEFT_OFFSET: f32 = 100.0;
static ENEMY_TOP_OFFSET: f32 = 100.0;
static ENEMY_DIST: f32 = 100.0;
static ENEMY_ROWS: usize = 5;
static ENEMIES_PER_ROW: usize = 7;
static ENEMY_SPEED_X: f32 = 1.0;
static ENEMY_SPEED_Y: f32 = 0.1;

static ENEMY_IMGS: [&str; 4] = [
    "/images/si_enemy_1_0.png",
    "/images/si_enemy_1_1.png",
    "/images/si_enemy_2_0.png",
    "/images/si_enemy_2_1.png",
];

pub struct EnemyId(usize, usize);

struct EnemyRow {
    enemies: Vec<Option<Enemy>>,
}

impl EnemyRow {
    fn new(ctx: &mut Context, image_cache: &mut ImageCache, row_idx: usize) -> EnemyRow {
        let mut rng = rand::thread_rng();

        let mut enemies = Vec::new();

        for x_index in 0..ENEMIES_PER_ROW {
            let img_index = rng.gen_range(0, ENEMY_IMGS.len());
            let enemy_image = image_cache.image(ctx, ENEMY_IMGS[img_index]).unwrap();

            let x_pos = ENEMY_DIST * (x_index as f32 + 1.0);
            let y_pos = (50.0 * row_idx as f32) + ENEMY_TOP_OFFSET;

            let mut enemy = Enemy::new(enemy_image, 0);
            enemy.transform(x_pos, y_pos);

            enemies.push(Some(enemy));
        }

        EnemyRow { enemies }
    }
}

pub struct EnemySystem {
    dir: f32,
    dir_tracker: TimedTracker,
    rows: Vec<EnemyRow>,
}

impl EnemySystem {
    pub fn new(ctx: &mut Context, image_cache: &mut ImageCache) -> EnemySystem {
        let mut rows = Vec::new();

        for y_index in 0..ENEMY_ROWS {
            rows.push(EnemyRow::new(ctx, image_cache, y_index));
        }

        EnemySystem {
            rows,
            dir: 1.0,
            dir_tracker: TimedTracker::new(Duration::from_millis(1000)),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        for row in self.rows.iter() {
            for enemy_wrapped in row.enemies.iter() {
                match enemy_wrapped {
                    Some(enemy) => enemy.draw(ctx)?,
                    None => {}
                }
            }
        }

        Ok(())
    }

    pub fn update(&mut self, player_box: &BoundingBox, screen_bounds: ScreenBounds) -> bool {
        if self.dir_tracker.can() {
            self.dir_tracker.track();
            self.dir *= -1.0;
        }

        let mut row_dir: f32 = self.dir;

        for row in self.rows.iter_mut() {
            // change direction of movement for each row
            row_dir *= -1.0;

            for enemy_wrapped in row.enemies.iter_mut() {
                match enemy_wrapped {
                    Some(enemy) => {
                        enemy.transform.x += (ENEMY_SPEED_X * row_dir);
                        enemy.transform.y += ENEMY_SPEED_Y;

                        if enemy.touched_bottom(screen_bounds) {
                            return true;
                        }

                        if enemy.collides_with(player_box) {
                            return true;
                        }
                    }
                    None => {}
                }
            }
        }

        false
    }

    pub fn collides_with(&self, other: &BoundingBox) -> Option<EnemyId> {
        for (row_idx, row) in self.rows.iter().enumerate() {
            for (enemy_idx, enemy_wrapped) in row.enemies.iter().enumerate() {
                match enemy_wrapped {
                    Some(enemy) => {
                        if enemy.collides_with(other) {
                            return Some(EnemyId(row_idx, enemy_idx));
                        }
                    }
                    None => {}
                }
            }
        }

        return None;
    }

    pub fn clear_enemy(&mut self, enemy_id: EnemyId) {
        self.rows[enemy_id.0].enemies[enemy_id.1] = None;
    }
}

pub struct Enemy {
    id: u16,
    image: Image,
    transform: Point2<f32>,
}

impl Enemy {
    pub fn new(image: Image, id: u16) -> Enemy {
        Enemy {
            id,
            image,
            transform: Point2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn transform(&mut self, x: f32, y: f32) {
        self.transform.x = x;
        self.transform.y = y;
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

    fn collides_with(&self, other: &BoundingBox) -> bool {
        return overlaps(&self.bb(), other);
    }

    fn touched_bottom(&self, screen_bounds: ScreenBounds) -> bool {
        let own_bottom = self.transform.y + self.bb().h;

        return own_bottom >= screen_bounds.1;
    }
}
