use crate::application::asset::texture::SrcRect;
use crate::application::geometry::Vec2;

pub type TileId = u32;

#[derive(Clone, Copy, Debug)]
pub struct TileData {
  pub id: TileId,
  pub src: SrcRect, // segment of the tileset to be rendered
}

#[derive(Clone, Debug)]
pub struct Tile {
  pub id: TileId,
  pub src: SrcRect,
  pub position: Vec2<i32>, // worldspace
}

impl Tile {
  pub fn new(data: TileData, position: Vec2<i32>) -> Self {
    let TileData { src, id } = data;
    Self { src, id, position }
  }
}
