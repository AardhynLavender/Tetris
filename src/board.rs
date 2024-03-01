use std::rc::Rc;

use sdl2::keyboard::Keycode;

use crate::application::event::EventStore;
use crate::application::geometry::Vec2;
use crate::application::manager::object::Object;
use crate::application::render::Renderer;
use crate::application::tile::{tile::TileData, tilemap::Tilemap, tileset::Tileset};
use crate::application::utility::types::{Coordinate, Size2};
use crate::constants::shape::ShapeType;
use crate::piece::{erase_piece, Piece, rotate_piece, Transform, transform_piece, write_piece};

const BOARD_DIMENSIONS: Size2 = Vec2::new(10, 20);
const BOARD_POSITION: Vec2<i32> = Vec2::new(1, 1);

pub struct Board {
  piece: Option<Piece>,
  tilemap: Tilemap,
}

impl Board {
  pub fn new(tileset: Rc<Tileset>) -> Self {
    let mut tilemap = Tilemap::new(Rc::clone(&tileset), BOARD_POSITION, BOARD_DIMENSIONS);
    Self { piece: None, tilemap }
  }

  pub fn render(&self, renderer: &mut Renderer) {
    for tile in &self.tilemap {
      if let Some(tile) = tile {
        let position = Vec2::new(tile.position.x + self.tilemap.position.x, tile.position.y + self.tilemap.position.y);
        renderer.draw_from_texture(&self.tilemap.tileset.texture, position, tile.src);
      }
    }
  }

  pub fn update(&mut self, events: &EventStore) {
    if let Some(piece) = &mut self.piece {
      // move
      if events.is_key_pressed(Keycode::S) {
        erase_piece(piece, &mut self.tilemap);
        transform_piece(piece, Transform::Down, &mut self.tilemap);
        write_piece(piece, &mut self.tilemap);
      }
      if events.is_key_pressed(Keycode::A) {
        erase_piece(piece, &mut self.tilemap);
        transform_piece(piece, Transform::Left, &mut self.tilemap);
        write_piece(piece, &mut self.tilemap);
      }
      if events.is_key_pressed(Keycode::D) {
        erase_piece(piece, &mut self.tilemap);
        transform_piece(piece, Transform::Right, &mut self.tilemap);
        write_piece(piece, &mut self.tilemap);
      }

      // rotate
      if events.is_key_pressed(Keycode::J) {
        erase_piece(piece, &mut self.tilemap);
        rotate_piece(piece, &mut self.tilemap);
        write_piece(piece, &mut self.tilemap);
      }
    }
  }

  pub fn spawn_piece(&mut self) {
    let piece = Piece::build(ShapeType::J, &*self.tilemap.tileset);
    write_piece(&piece, &mut self.tilemap);
    self.piece = Some(piece);
  }

  fn move_lines_down(&mut self, line: usize) -> Result<(), String> {
    if line > BOARD_DIMENSIONS.y as usize {
      return Err(String::from("line move out of bounds"));
    }
    if line == BOARD_DIMENSIONS.y as usize {
      return Err(String::from("cannot move last line"));
    }

    // loop the lines from the line to the top
    for y in (0..line).rev() {
      for x in 0..BOARD_DIMENSIONS.x {
        // get tile
        let coord = Coordinate::new(x as i32, y as i32);
        let tile = self.tilemap.get_at_coord(&coord);
        if let Some(tile) = tile {
          // move tile down
          let data = TileData { id: tile.id, src: tile.src };
          let new_coord = Coordinate::new(x as i32, y as i32 + 1);
          self.tilemap.set_tile_at_coord(&new_coord, data);
        }

        // clear old tile
        self.tilemap.clear_tile_at_coord(&coord);
      }
    }

    Ok(())
  }

  pub fn clear_line(&mut self, line: usize) -> Result<(), String> {
    if line >= BOARD_DIMENSIONS.y as usize {
      return Err(String::from("line clear out of bounds"));
    }

    for x in 0..BOARD_DIMENSIONS.x {
      let coord = Coordinate::new(x as i32, line as i32);
      self.tilemap.clear_tile_at_coord(&coord);
    }

    Ok(())
  }
}