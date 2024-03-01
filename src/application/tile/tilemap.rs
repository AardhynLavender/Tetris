use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::application::event::EventStore;
use crate::application::geometry::Vec2;
use crate::application::manager::object::Object;
use crate::application::render::Renderer;
use crate::application::tile::tile::{Tile, TileData};
use crate::application::tile::tileset::Tileset;
use crate::application::utility::container::{coordinate_to_index, index_to_coordinate};
use crate::application::utility::types::{Coordinate, Size2};

// Store //

pub struct TilemapStore {
  tilemaps: HashMap<String, Tilemap>,
}

impl TilemapStore {
  pub fn new() -> Self {
    Self { tilemaps: HashMap::new() }
  }
  pub fn add(&mut self, name: String, tilemap: Tilemap) -> &mut Tilemap {
    self.tilemaps.entry(name).or_insert(tilemap)
  }
  pub fn get(&mut self, name: &str) -> Result<&Tilemap, &str> {
    match self.tilemaps.get_mut(name) {
      Some(tilemap) => Ok(tilemap),
      None => Err("Tilemap not found"),
    }
  }
  pub fn get_mut(&mut self, name: &str) -> Result<&mut Tilemap, &str> {
    match self.tilemaps.get_mut(name) {
      Some(tilemap) => Ok(tilemap),
      None => Err("Tilemap not found"),
    }
  }
}

// Tilemap //

pub type MapData = Vec<Option<Tile>>;

/*
 * todo: reference or point to a tileset as multiple tilemaps can share the same tileset.
 *       further, a single tilemap may need to reference multiple tilesets.
 */
pub struct Tilemap {
  pub tileset: Rc<Tileset>,
  tiles: MapData,
  pub position: Vec2<i32>,
  pub dimensions: Size2,
}

impl<'a> Tilemap {
  pub fn new(tileset: Rc<Tileset>, position: Vec2<i32>, dimensions: Size2) -> Self {
    let size_tiles = dimensions.x * dimensions.y;
    let tiles: MapData = vec![None; size_tiles as usize];
    Self { tileset, tiles, position, dimensions }
  }

  // accessors //

  pub fn get_at_coord(&self, coordinate: &Coordinate) -> Option<&Tile> {
    let index = coordinate_to_index(coordinate, self.dimensions);
    self.get_at_index(index)
  }
  pub fn get_at_index(&self, index: usize) -> Option<&Tile> {
    if let Some(tile) = self.tiles.get(index) {
      return tile.as_ref();
    }
    None
  }

  // mutation //

  pub fn set_tile_at_coord(&mut self, coordinate: &Coordinate, data: TileData) {
    let position = self.coord_to_worldspace(&coordinate);
    let tile = Tile::new(data, position);
    let index = coordinate_to_index(&coordinate, self.dimensions);
    self.tiles[index] = Some(tile);
  }
  pub fn set_tile_at_index(&mut self, index: usize, data: TileData) {
    let dimensions = Coordinate::new(self.dimensions.x as i32, self.dimensions.y as i32);
    let coordinate = index_to_coordinate(index, &dimensions);
    self.set_tile_at_coord(&coordinate, data);
  }

  pub fn clear_tile_at_coord(&mut self, coordinate: &Coordinate) {
    let index = coordinate_to_index(&coordinate, self.dimensions);
    self.tiles[index] = None;
  }
  pub fn clear_tile_at_index(&mut self, index: usize) {
    let dimensions = Coordinate::new(self.dimensions.x as i32, self.dimensions.y as i32);
    let coordinate = index_to_coordinate(index, &dimensions);
    self.clear_tile_at_coord(&coordinate);
  }

  // conversion //

  fn coord_to_worldspace(&self, coordinate: &Coordinate) -> Vec2<i32> {
    let (tile_width, tile_height) = self.tileset.tile_size.destructure();
    Vec2::new(
      self.position.x + (coordinate.x * tile_width as i32),
      self.position.y + (coordinate.y * tile_height as i32),
    )
  }
  fn index_to_worldspace(&self, index: usize) -> Vec2<i32> {
    let dimensions = Coordinate::new(self.dimensions.x as i32, self.dimensions.y as i32);
    let coordinate = index_to_coordinate(index, &dimensions);
    self.coord_to_worldspace(&coordinate)
  }

  // Queries //

  pub fn is_bound(&self, coordinate: &Coordinate) -> bool {
    let x_bound = coordinate.x >= 0 && coordinate.x < self.dimensions.x as i32;
    let y_bound = coordinate.y >= 0 && coordinate.y < self.dimensions.y as i32;
    x_bound && y_bound
  }

  pub fn is_occupied(&self, coordinate: &Coordinate) -> bool {
    self.get_at_coord(coordinate).is_some()
  }
}

impl<TState> Object<TState> for Tilemap {
  fn update(&mut self, _: &mut TState) {}
  fn render(&self, renderer: &mut Renderer) {}
  fn event(&mut self, _: &EventStore) {}
}

// iterate over tile
impl<'a> IntoIterator for &'a Tilemap {
  type Item = &'a Option<Tile>;
  type IntoIter = std::slice::Iter<'a, Option<Tile>>;

  fn into_iter(self) -> Self::IntoIter {
    self.tiles.iter()
  }
}

// iterate over mutable tile
impl<'a> IntoIterator for &'a mut Tilemap {
  type Item = &'a mut Option<Tile>;
  type IntoIter = std::slice::IterMut<'a, Option<Tile>>;

  fn into_iter(self) -> Self::IntoIter {
    self.tiles.iter_mut()
  }
}