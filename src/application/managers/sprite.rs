use std::collections::HashMap;

use crate::application::tiles::tilemap::Tilemap;

pub enum SpriteType {
  Tilemap(Tilemap),
}

pub struct SpriteManager {
  pub sprites: HashMap<String, SpriteType>,
}

impl SpriteManager {
  pub fn new() -> Self {
    Self { sprites: HashMap::new() }
  }
  pub fn add(&mut self, name: String, sprite: SpriteType) -> &mut SpriteType {
    self.sprites.entry(name).or_insert(sprite)
  }
  pub fn get(&self, name: &str) -> Option<&SpriteType> {
    match self.sprites.get(name) {
      Some(sprite) => Some(sprite),
      None => None,
    }
  }
  pub fn get_mut(&mut self, name: &str) -> Option<&mut SpriteType> {
    match self.sprites.get_mut(name) {
      Some(sprite) => Some(sprite),
      None => None,
    }
  }
  pub fn remove(&mut self, name: &str) -> Option<SpriteType> {
    self.sprites.remove(name)
  }
}