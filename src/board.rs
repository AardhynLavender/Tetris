use std::rc::Rc;

use sdl2::keyboard::Keycode;

use crate::application::event::EventStore;
use crate::application::geometry::{Rec2, Vec2};
use crate::application::render::color::color;
use crate::application::render::Renderer;
use crate::application::tile::{tile::TileData, tilemap::Tilemap, tileset::Tileset};
use crate::application::utility::types::{Coordinate, Size, Size2};
use crate::constants::shape::ShapeType;
use crate::piece::{erase_piece, Piece, rotate_piece, Transform, transform_piece, write_piece};

const BOARD_DIMENSIONS: Size2 = Vec2::new(10, 20);
const BORDER_MARGIN: Size = 2;
const BOARD_POSITION: Vec2<i32> = Vec2::new(384 / 2 - BOARD_DIMENSIONS.x as i32 * 8 / 2, 216 / 2 - BOARD_DIMENSIONS.y as i32 * 8 / 2);
const TILE_PIECE_MARGIN: Size = 1; // margin between pieces in the tileset

pub struct Board {
  piece: Option<Piece>,
  tilemap: Tilemap,
  border: Size2,
}

impl Board {
  pub fn new(tileset: Rc<Tileset>) -> Self {
    let mut tilemap = Tilemap::new(Rc::clone(&tileset), BOARD_POSITION, BOARD_DIMENSIONS);
    let (w, h) = tilemap.dimensions.destructure();
    let (tiles_x, tiles_y) = tileset.tile_size.destructure();
    let border = Vec2::new(w * tiles_x + BORDER_MARGIN + TILE_PIECE_MARGIN, h * tiles_y + BORDER_MARGIN + TILE_PIECE_MARGIN);
    Self { piece: None, tilemap, border }
  }

  pub fn render(&self, renderer: &mut Renderer) {
    // draw tiles
    for tile in &self.tilemap {
      if let Some(tile) = tile {
        let position = Vec2::new(tile.position.x, tile.position.y);
        renderer.draw_from_texture(&self.tilemap.tileset.texture, position, tile.src);
      }
    }

    // draw border
    let border_position = Vec2::new(self.tilemap.position.x - BORDER_MARGIN as i32, self.tilemap.position.y - BORDER_MARGIN as i32);
    let rect = Rec2::new(border_position, self.border);
    renderer.draw_rect(rect, color::SURFACE_0);
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