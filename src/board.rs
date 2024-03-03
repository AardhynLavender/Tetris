use std::rc::Rc;

use sdl2::keyboard::Keycode;

use crate::application::event::EventStore;
use crate::application::geometry::{Rec2, Vec2};
use crate::application::render::color::color;
use crate::application::render::Renderer;
use crate::application::tile::tile::TileData;
use crate::application::tile::tilemap::Tilemap;
use crate::application::tile::tileset::Tileset;
use crate::application::time::{Repeat, Timeout};
use crate::application::utility::types::{Coordinate, Size2};
use crate::constants::board::{BOARD_DIMENSIONS, BOARD_POSITION, BORDER_MARGIN, TILE_PIECE_MARGIN};
use crate::constants::game::FALL_COOLDOWN;
use crate::constants::piece::ShapeType;
use crate::piece::{erase_piece, Piece, PieceState, rotate_piece, Transform, transform_piece, write_piece};

pub struct Board {
  piece: Option<Piece>,
  tilemap: Tilemap,
  drop_timeout: Timeout,
  border: Size2,
}

impl Board {
  pub fn new(tileset: Rc<Tileset>) -> Self {
    let mut tilemap = Tilemap::new(Rc::clone(&tileset), BOARD_POSITION, BOARD_DIMENSIONS);
    let (w, h) = tilemap.dimensions.destructure();
    let (tiles_x, tiles_y) = tileset.tile_size.destructure();

    let border = Vec2::new(w * tiles_x + BORDER_MARGIN + TILE_PIECE_MARGIN, h * tiles_y + BORDER_MARGIN + TILE_PIECE_MARGIN);

    Self {
      piece: None,
      drop_timeout: Timeout::new(FALL_COOLDOWN, Repeat::Forever),
      tilemap,
      border,
    }
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
      erase_piece(piece, &mut self.tilemap); // erase the old piece

      // move the piece down
      self.drop_timeout.on_timeout(&mut || {
        transform_piece(piece, Transform::Down, &mut self.tilemap);
      });

      let player_can_move = piece.player_transform_cooldown.consume();
      let down = events.is_key_held(Keycode::S) && player_can_move;
      let left = events.is_key_held(Keycode::A) && player_can_move;
      let right = events.is_key_held(Keycode::D) && player_can_move;

      if down {
        transform_piece(piece, Transform::Down, &mut self.tilemap);
        self.drop_timeout.reset(); // reset the computer drop timeout
        piece.player_transform_cooldown.reset(); // reset the computer drop timeout
      }
      if left {
        transform_piece(piece, Transform::Left, &mut self.tilemap);
        piece.player_transform_cooldown.reset(); // reset the computer drop timeout
      }
      if right {
        transform_piece(piece, Transform::Right, &mut self.tilemap);
        piece.player_transform_cooldown.reset(); // reset the computer drop timeout
      }

      // rotate
      if events.is_key_pressed(Keycode::J) {
        rotate_piece(piece, &mut self.tilemap);
      }

      write_piece(piece, &mut self.tilemap); // write the new piece

      if piece.state == PieceState::Landed {
        let full_lines = self.get_full_line();
        for line in full_lines {
          self.clear_line(line).expect("failed to clear line");
          self.move_lines_down(line).expect("failed to move lines down");
        }
        self.spawn_piece();
      }
    } else {
      self.spawn_piece();
    }
  }

  pub fn spawn_piece(&mut self) {
    let piece = Piece::build(ShapeType::random(), &*self.tilemap.tileset);
    write_piece(&piece, &mut self.tilemap);
    self.piece = Some(piece);
  }

  pub fn move_lines_down(&mut self, line: usize) -> Result<(), String> {
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

  fn get_full_line(&self) -> Vec<usize> {
    // todo: recreate this declaratively using array methods
    let mut full_lines = Vec::new();
    for y in 0..BOARD_DIMENSIONS.y {
      let mut line_full = true;
      for x in 0..BOARD_DIMENSIONS.x {
        let coord = Coordinate::new(x as i32, y as i32);
        if !self.tilemap.is_occupied(&coord) {
          line_full = false;
          break;
        }
      }
      if line_full {
        full_lines.push(y as usize);
      }
    }

    full_lines
  }

  fn clear_line(&mut self, line: usize) -> Result<(), String> {
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