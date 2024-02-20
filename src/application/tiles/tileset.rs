use std::collections::HashMap;
use std::rc::Rc;

use crate::application::asset::texture::Texture;
use crate::application::geometry::{Rec2, Vec2};
use crate::application::tiles::tile::{TileData, TileId};
use crate::application::utility::types::Size2;

// Store //

pub struct TilesetStore {
  tilesets: HashMap<String, Rc<Tileset>>,
}

impl TilesetStore {
  pub fn new() -> Self {
    Self { tilesets: HashMap::new() }
  }
  pub fn add(&mut self, name: String, tileset: Rc<Tileset>) -> Rc<Tileset> {
    Rc::clone(self.tilesets.entry(name).or_insert(tileset))
  }
  pub fn get(&self, name: &str) -> Result<Rc<Tileset>, &str> {
    self.tilesets.get(name).map(Rc::clone).ok_or("Failed to get tileset")
  }
}

// Tileset //

pub struct Tileset {
  pub texture: Rc<Texture>,
  pub tile_size: Vec2<u32>,
  pub tiles: Vec<TileData>,
}

impl Tileset {
  pub fn new(texture: Rc<Texture>, tile_size: Size2) -> Self {
    let tiles = make_tiles(texture.dimensions, tile_size).expect("Failed to make tiles");
    Self { texture, tile_size, tiles }
  }

  pub fn get_tiledata(&self, id: TileId) -> Option<TileData> {
    if let Some(tile) = self.tiles.get(id as usize) {
      return Some(*tile).clone();
    }
    None
  }
}

fn make_tiles(dimensions: Size2, tile_size: Size2) -> Result<Vec<TileData>, &'static str> {
  let (width, height) = dimensions.destructure();

  if width % tile_size.x != 0 {
    return Err("Tileset width must be divisible by tile size");
  }
  if height % tile_size.y != 0 {
    return Err("Tileset height must be divisible by tile size");
  }

  let mut tiles = Vec::new();
  for y in 0..width / tile_size.y {
    for x in 0..width / tile_size.x {
      let id = (y * (width / tile_size.x) + x);
      let tile_position = Vec2::new(x * tile_size.x, y * tile_size.y);
      let src = Rec2::new(tile_position, tile_size);
      tiles.push(TileData { id, src });
    }
  }
  Ok(tiles)
}