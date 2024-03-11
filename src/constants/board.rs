use crate::engine::geometry::Vec2;
use crate::engine::render::color::color;
use crate::engine::render::color::RGBA;
use crate::engine::utility::types::{Size, Size2};

pub const TILE_SIZE: Size = 8;
pub const TILE_PIECE_MARGIN: Size = 1; // margin between pieces in the tileset

pub const BORDER_MARGIN: Size = 2;
pub const BORDER_COLOR: RGBA = color::SURFACE_0;
pub const BOARD_POSITION: Vec2<i32> = Vec2::new(10, 10);
pub const BOARD_DIMENSIONS: Size2 = Vec2::new(10, 20);

pub const FIRST_ROW: i32 = 0;
pub const SPAWN_OFFSET_X: i32 = 4; // center the piece on the board.rs
