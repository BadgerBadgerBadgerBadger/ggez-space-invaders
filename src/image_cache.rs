use ggez::graphics::Image;
use ggez::{Context, GameResult};
use std::collections::HashMap;

pub struct ImageCache {
    images: HashMap<&'static str, Image>,
}

impl ImageCache {
    pub fn new() -> ImageCache {
        ImageCache {
            images: HashMap::new(),
        }
    }

    pub fn image(&mut self, ctx: &mut Context, path: &'static str) -> GameResult<Image> {
        return match self.images.get(path) {
            Some(image) => return Ok(image.clone()),
            None => {
                let image = Image::new(ctx, path)?;

                self.images.insert(path, image.clone());

                return Ok(image);
            }
        };
    }
}
