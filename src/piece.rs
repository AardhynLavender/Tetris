use crate::algorithm::{check_bounds, check_shape_collision, get_new_shape_coordinates, is_shape_on_bottom, transform_shape};
use crate::application::tile::{tile::TileData, tilemap::Tilemap};
use crate::application::tile::tileset::Tileset;
use crate::application::time::Timer;
use crate::application::utility::types::Coordinate;
use crate::constants::game::{PLAYER_DROP_COOLDOWN, PLAYER_SLIDE_COOLDOWN};
use crate::constants::piece::{DEFAULT_ROTATION, Shape, ShapeData, ShapeType};

const SPAWN_OFFSET_X: i32 = 4; // center the piece on the board.rs

const FIRST_ROW: i32 = 0;

#[derive(Debug, PartialEq)]
pub enum PieceState {
  /// The piece is still active and can be transformed.
  Active,
  /// The piece has landed and is no longer active.
  Landed,
}

#[derive(Debug)]
pub struct Piece {
  pub shape_type: ShapeType,
  pub state: PieceState,
  pub shape_data: ShapeData,
  pub tile_data: TileData,

  pub rotation: usize,
  pub position: Coordinate,

  pub player_slide_cooldown: Timer,
  pub player_drop_cooldown: Timer,
}

impl Piece {
  pub fn build(shape_type: ShapeType, tileset: &Tileset) -> Self {
    let piece_data = shape_type.data();
    let tile_data = tileset.get_tiledata(piece_data.tile_id).expect("failed to get tile data");
    let position = Coordinate::new(SPAWN_OFFSET_X, FIRST_ROW - piece_data.offset_y as i32);

    Self {
      shape_type,
      state: PieceState::Active,
      tile_data,
      shape_data: piece_data.shape,

      rotation: DEFAULT_ROTATION,
      position,

      player_slide_cooldown: Timer::new(PLAYER_SLIDE_COOLDOWN, true),
      player_drop_cooldown: Timer::new(PLAYER_DROP_COOLDOWN, true),
    }
  }
}

// writers //

pub fn write_piece(piece: &Piece, tilemap: &mut Tilemap) {
  for coord in &piece.shape_data[piece.rotation] {
    let position = Coordinate::new(piece.position.x + coord.x, piece.position.y + coord.y);
    tilemap.set_tile_at_coord(&position, piece.tile_data);
  }
}

pub fn erase_piece(piece: &Piece, tilemap: &mut Tilemap) {
  for coord in &piece.shape_data[piece.rotation] {
    let position = Coordinate::new(piece.position.x + coord.x, piece.position.y + coord.y);
    tilemap.clear_tile_at_coord(&position);
  }
}

// Transform //

#[derive(PartialEq, Debug)]
pub enum Transform { Left, Right, Down }

impl Transform {
  pub fn to_coordinate(&self) -> Coordinate {
    match self {
      Transform::Left => Coordinate::new(-1, 0),
      Transform::Right => Coordinate::new(1, 0),
      Transform::Down => Coordinate::new(0, 1),
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum TransformResult {
  /// The piece can be transformed.
  Success { position: Coordinate },
  /// The piece is unable to move due to a collision with shape or bounds.
  Collision,
  /// The piece will land and become inactive.
  Land,
}

/// Check if the piece can be transformed
fn evaluate_transform(piece: &Piece, transform: Transform, tilemap: &Tilemap) -> TransformResult {
  let shape = &piece.shape_data[piece.rotation];
  let transform_coord = transform.to_coordinate();
  let new_position = Coordinate::new(piece.position.x + transform_coord.x, piece.position.y + transform_coord.y);
  let new_shape = transform_shape(&shape, &new_position);

  // check bounds
  let unchecked_coordinates: Shape = get_new_shape_coordinates(shape, &new_shape);
  let is_bound = check_bounds(&unchecked_coordinates, tilemap);

  if !is_bound {
    let on_bottom = is_shape_on_bottom(&unchecked_coordinates, tilemap);
    if transform == Transform::Down && on_bottom {
      // if we're moving down and out of bounds, we've landed
      return TransformResult::Land;
    }
    return TransformResult::Collision;
  }

  // check shape collision
  let is_collision = check_shape_collision(&unchecked_coordinates, tilemap);
  if is_collision {
    // if we're moving down and there's a collision, we've landed
    if transform == Transform::Down {
      return TransformResult::Land;
    }
    return TransformResult::Collision;
  }

  TransformResult::Success { position: new_position }
}

/// Apply the transform to the piece if possible
pub fn transform_piece(piece: &mut Piece, event: Transform, tilemap: &mut Tilemap) -> PieceState {
  match evaluate_transform(piece, event, tilemap) {
    TransformResult::Success { position } => {
      piece.position = position;
      piece.state = PieceState::Active;
      PieceState::Active
    }
    TransformResult::Land => {
      piece.state = PieceState::Landed;
      PieceState::Landed
    }
    TransformResult::Collision => {
      PieceState::Active
    }
  }
}

// Rotation //

pub enum RotationResult {
  /// The piece can be rotated.
  Success { rotation: usize },
  /// The piece is unable to move due to a collision with shape or bounds.
  Collision,
}

/// Check if the piece can be rotated
fn evaluate_rotation(piece: &Piece, tilemap: &Tilemap) -> RotationResult {
  let shape = &piece.shape_data[piece.rotation];
  let shape_rotations = piece.shape_type.data().shape;

  // get new shape
  let new_rotation = (piece.rotation + 1) % shape_rotations.len();
  let new_shape: Shape = shape_rotations[new_rotation].iter()
    .map(|coord| Coordinate::new(coord.x + piece.position.x, coord.y + piece.position.y))
    .collect();

  // check bounds
  let unchecked_coordinates: Shape = get_new_shape_coordinates(&shape, &new_shape);
  let is_bound = check_bounds(&unchecked_coordinates, tilemap);
  if !is_bound {
    return RotationResult::Collision;
  }

  // check shape collision
  let is_collision = check_shape_collision(&unchecked_coordinates, tilemap);
  if is_collision {
    return RotationResult::Collision;
  }

  RotationResult::Success {
    rotation: new_rotation,
  }
}

/// Apply the rotation to the piece if possible
pub fn rotate_piece(piece: &mut Piece, tilemap: &mut Tilemap) -> PieceState {
  if let RotationResult::Success { rotation } = evaluate_rotation(piece, tilemap) {
    piece.rotation = rotation;
  }
  return PieceState::Active; // shape is always active after rotation
}