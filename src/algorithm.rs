use crate::constants::game::{DOUBLE_LINE_MULTIPLIER, FALL_COOLDOWN, LEVEL_SPEED_MULTIPLIER, MAX_LINES, MAX_TETRIS_LEVEL, SINGLE_LINE_MULTIPLIER, START_TETRIS_LEVEL, TETRIS_MULTIPLIER, TRIPLE_LINE_MULTIPLIER};
use crate::constants::piece::Shape;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::utility::types::Coordinate;

/**
 * utility algorithms for Tetris
 */

// Coordinate //

/// Check if a coordinate is above the bounds of a tilemap
pub fn above_bounds(coordinate: &Coordinate, tilemap: &Tilemap) -> bool {
  let is_bound_horizontal = coordinate.x >= 0 && coordinate.x < tilemap.dimensions.x as i32;
  coordinate.y < 0 && is_bound_horizontal
}

/// Check if a shape is on the bottom of a tilemap
pub fn is_shape_on_bottom(coordinate: &Shape, tilemap: &Tilemap) -> bool {
  let last_row = tilemap.dimensions.y as i32 - 1;
  coordinate.iter()
    .any(|c| c.y >= last_row)
}

// Shape //

/// Transform the coordinates of a shape by a coordinate
pub fn transform_shape(shape: &Shape, transform: &Coordinate) -> Shape {
  let (x, y) = transform.destructure();
  shape.iter()
    .map(|coord| { Coordinate::new(coord.x + x, coord.y + y) })
    .collect()
}

/// Get the coordinates in `new` that are not in `old`
pub fn get_new_shape_coordinates<'a>(old: &'a Shape, new: &'a Shape) -> Shape {
  new.iter()
    .filter(|coord| !old.contains(coord))
    .map(|coord| *coord)
    .collect()
}

/// Check if a shape is wholly contained the bounds of a  tilemap
pub fn check_bounds(shape: &Shape, tilemap: &Tilemap) -> bool {
  shape.iter()
    .all(|c| above_bounds(c, tilemap) || tilemap.is_bound(c))
}

/// Check if any coordinate of `shape` is occupied in `tilemap`
pub fn check_shape_collision(shape: &Shape, tilemap: &Tilemap) -> bool {
  shape.iter()
    .any(|c| tilemap.is_occupied(c))
}

// Score and Level //

pub fn level_invariant(level: u32) -> Result<(), String> {
  if level < START_TETRIS_LEVEL {
    return Err(String::from("Level must be at least 1"));
  }
  if level > MAX_TETRIS_LEVEL {
    return Err(String::from("Level must be at most 29"));
  }
  Ok(())
}

pub fn calculate_score(lines: u32, level: u32) -> Result<u32, String> {
  level_invariant(level)?;

  if lines == 0 {
    return Ok(0);
  }

  let score = match lines {
    1 => SINGLE_LINE_MULTIPLIER,
    2 => DOUBLE_LINE_MULTIPLIER,
    3 => TRIPLE_LINE_MULTIPLIER,
    4 => TETRIS_MULTIPLIER,
    _ => return Err(String::from("Invalid number of lines cleared"))
  };

  Ok(score * level)
}

pub fn calculate_speed_ms(level: u32) -> Result<u64, String> {
  level_invariant(level)?;
  let speed = (FALL_COOLDOWN.as_millis() as f32 * (LEVEL_SPEED_MULTIPLIER.powf(level as f32))) as u64;
  Ok(speed)
}

pub fn determine_sfx(lines: u32) -> Option<&'static str> {
  if lines > 0 && lines < MAX_LINES {
    return Some("line");
  } else if lines == MAX_LINES {
    return Some("tetris");
  }
  return None;
}
