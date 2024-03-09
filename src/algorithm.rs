use crate::application::tile::tilemap::Tilemap;
use crate::application::utility::types::Coordinate;
use crate::constants::piece::Shape;

pub fn above_bounds(coordinate: &Coordinate, tilemap: &Tilemap) -> bool {
  let is_bound_horizontal = coordinate.x >= 0 && coordinate.x < tilemap.dimensions.x as i32;
  coordinate.y < 0 && is_bound_horizontal
}

pub fn is_shape_on_bottom(coordinate: &Shape, tilemap: &Tilemap) -> bool {
  let last_row = tilemap.dimensions.y as i32 - 1;
  coordinate.iter()
    .any(|c| c.y >= last_row)
}

pub fn transform_shape(shape: &Shape, transform: &Coordinate) -> Shape {
  let (x, y) = transform.destructure();
  shape.iter()
    .map(|coord| { Coordinate::new(coord.x + x, coord.y + y) })
    .collect()
}

pub fn get_new_shape_coordinates<'a>(old: &'a Shape, new: &'a Shape) -> Shape {
  new.iter()
    .filter(|coord| !old.contains(coord))
    .map(|coord| *coord)
    .collect()
}

pub fn check_bounds(shape: &Shape, tilemap: &Tilemap) -> bool {
  shape.iter()
    .all(|c| above_bounds(c, tilemap) || tilemap.is_bound(c))
}

pub fn check_shape_collision(shape: &Shape, tilemap: &Tilemap) -> bool {
  shape.iter()
    .any(|c| tilemap.is_occupied(c))
}
