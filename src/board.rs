use std::rc::Rc;
use std::time::Duration;

use sdl2::keyboard::Keycode;

use crate::constants::board::{BOARD_DIMENSIONS, BOARD_POSITION, BORDER_COLOR, BORDER_MARGIN, TILE_PIECE_MARGIN};
use crate::constants::game::START_TETRIS_LEVEL;
use crate::constants::piece::ShapeType;
use crate::engine::event::EventStore;
use crate::engine::geometry::{Rec2, Vec2};
use crate::engine::render::Renderer;
use crate::engine::tile::tile::TileData;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::tile::tileset::Tileset;
use crate::engine::time::{Repeat, Timeout};
use crate::engine::utility::types::{Coordinate, Size2};
use crate::math::calculate_speed_ms;
use crate::piece::{erase_piece, Piece, PieceState, rotate_piece, Transform, transform_piece, write_piece};

pub enum SpawnState {
  Space,
  Occupied,
}

pub enum BoardEvent {
  MoveLeft,
  MoveRight,
  Rotate,
  Land,
  Nothing,
}

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

    let staring_fall_speed = calculate_speed_ms(START_TETRIS_LEVEL).expect("failed to calculate starting fall speed");

    Self {
      piece: None,
      drop_timeout: Timeout::new(Duration::from_millis(staring_fall_speed), Repeat::Forever),
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
    renderer.draw_rect(rect, BORDER_COLOR);
  }

  pub fn update(&mut self, events: &EventStore) -> BoardEvent {
    let mut board_event = BoardEvent::Nothing;

    if let Some(piece) = &mut self.piece {
      erase_piece(piece, &mut self.tilemap); // erase the old piece

      // move the piece down
      self.drop_timeout.on_timeout(&mut || {
        if transform_piece(piece, Transform::Down, &mut self.tilemap) == PieceState::Landed {
          board_event = BoardEvent::Land;
        }
      });

      let player_can_slide = piece.player_slide_cooldown.done();
      let player_can_drop = piece.player_drop_cooldown.done();
      let down = events.is_key_held(Keycode::S) && player_can_drop;
      let left = events.is_key_held(Keycode::A) && player_can_slide;
      let right = events.is_key_held(Keycode::D) && player_can_slide;
      let rotate = events.is_key_pressed(Keycode::J);

      // rotate
      if rotate {
        rotate_piece(piece, &mut self.tilemap);
        board_event = BoardEvent::Rotate;
      }

      // slide
      if left {
        transform_piece(piece, Transform::Left, &mut self.tilemap);
        piece.player_slide_cooldown.restart(); // reset the player slide cooldown
        board_event = BoardEvent::MoveLeft;
      }
      if right {
        transform_piece(piece, Transform::Right, &mut self.tilemap);
        piece.player_slide_cooldown.restart(); // reset the player slide cooldown
        board_event = BoardEvent::MoveRight;
      }

      // move down
      if down {
        if transform_piece(piece, Transform::Down, &mut self.tilemap) == PieceState::Landed {
          board_event = BoardEvent::Land;
        }
        self.drop_timeout.reset(); // reset the computer drop timeout
        piece.player_drop_cooldown.restart(); // reset the player drop cooldown
      }

      write_piece(piece, &mut self.tilemap); // write the new piece
    }

    return board_event;
  }

  /// Checks if the current `piece` has landed
  pub fn drop_complete(&self) -> bool {
    if let Some(piece) = &self.piece {
      return piece.state == PieceState::Landed;
    }
    false
  }

  /// Set the speed the computer drops a shape
  pub fn set_speed_ms(&mut self, speed_ms: u64) {
    self.drop_timeout = Timeout::new(Duration::from_millis(speed_ms), Repeat::Forever);
  }

  /// Reset `Piece` to a new random shape and check if it can be spawned
  pub fn spawn_piece(&mut self) {
    let piece = Piece::build(ShapeType::random(), &*self.tilemap.tileset);

    // todo: spawn check

    write_piece(&piece, &mut self.tilemap);
    self.piece = Some(piece);
    self.drop_timeout.reset();
  }

  /// Set the `Piece` to `None`
  pub fn kill_piece(&mut self) {
    self.piece = None;
  }

  /// Transform tiles on lines above `line` by {0, 1}
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

  /// Get lines containing only `Some` tiles
  pub fn get_full_lines(&self) -> Vec<usize> {
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

  /// Set all tiles in a line to `None`
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