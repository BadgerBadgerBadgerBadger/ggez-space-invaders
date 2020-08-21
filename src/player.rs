use crate::projectile::ProjectileSystem;
use crate::projectile::ShotType::PlayerShot;
use crate::timed_tracker::TimedTracker;
use ggez::graphics::mint::Point2;
use ggez::graphics::{mint, DrawParam, Image, Rect};
use ggez::mint::Vector2;
use ggez::{graphics, Context, GameResult};
use rand::Rng;
use std::time::{Duration, Instant};

static PLAYERS_SPEED_BASE: f32 = 2.5;
static PLAYERS_SPEED_BOOST: f32 = 7.0;
static PLAYERS_SPEED_BOOST_DRAIN: f32 = 0.5;
static PLAYER_SHOT_INTERVAL: u64 = 500;
static PLAYER_BOOST_INTERVAL: u64 = 1500;

pub enum Direction {
    Left,
    Right,
}

pub struct Player {
    pub image: Image,
    pub transform: Vector2<f32>,
    shot_tracker: TimedTracker,
    boost_tracker: TimedTracker,
    speed: f32,
    boosted_speed: f32,
}

impl Player {
    pub fn new(ctx: &mut Context, img_path: &str) -> Player {
        Player {
            image: Image::new(ctx, img_path).unwrap(),
            transform: Vector2 { x: 0.0, y: 0.0 },
            shot_tracker: TimedTracker::new(Duration::from_millis(PLAYER_SHOT_INTERVAL)),
            boost_tracker: TimedTracker::new(Duration::from_millis(PLAYER_BOOST_INTERVAL)),
            speed: PLAYERS_SPEED_BASE,
            boosted_speed: 0.0,
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.transform.x = x;
        self.transform.y = y;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.image, DrawParam::new().dest(self.transform))
    }

    pub fn center_x(&self) -> f32 {
        return self.transform.x + (self.image.dimensions().w / 2.0);
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

    pub fn move_player(&mut self, direction: Direction) {
        let mut dir_sign = match direction {
            Direction::Right => 1.0,
            Direction::Left => -1.0,
        };

        self.transform.x += (self.speed + self.boosted_speed) * dir_sign;

        if self.boosted_speed > 0.0 {
            self.boosted_speed -= PLAYERS_SPEED_BOOST_DRAIN;
        }
    }

    pub fn shoot(&mut self, ctx: &mut Context, projectile_system: &mut ProjectileSystem) {
        if !self.shot_tracker.can() {
            return;
        }

        let shot_from = Vector2 {
            x: self.center_x(),
            y: self.transform.y,
        };

        projectile_system.add_projectile(ctx, PlayerShot, shot_from);
        self.shot_tracker.track();
    }

    pub fn boost(&mut self) {
        if !self.boost_tracker.can() {
            return;
        }

        self.boosted_speed = PLAYERS_SPEED_BOOST;
        self.boost_tracker.track();
    }
}
