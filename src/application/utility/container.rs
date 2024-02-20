use crate::application::utility::types::{Coordinate, Size, Size2};

pub fn coordinate_to_index(position: Coordinate, dimensions: Size2) -> usize {
  (dimensions.x * position.y + position.x) as usize
}

pub fn index_to_coordinate(index: usize, dimensions: Size2) -> Coordinate {
  let x = index % dimensions.x as usize;
  let y = index / dimensions.x as usize;
  Coordinate::new(x as Size, y as Size)
}