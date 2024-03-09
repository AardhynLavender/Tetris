use crate::engine::utility::types::{Coordinate, Size2};

pub fn coordinate_to_index(position: &Coordinate, dimensions: Size2) -> usize {
  (dimensions.x as i32 * position.y + position.x) as usize
}

pub fn index_to_coordinate(index: usize, dimensions: &Coordinate) -> Coordinate {
  let x = index % dimensions.x as usize;
  let y = index / dimensions.x as usize;
  Coordinate::new(x as i32, y as i32)
}