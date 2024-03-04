use crate::application::geometry::Vec2;
use crate::application::utility::types::{Size, Size2};
use crate::constants::window::SCREEN_PIXELS;

pub const TILE_SIZE: Size = 8;
pub const TILE_PIECE_MARGIN: Size = 1; // margin between pieces in the tileset

pub const SCREEN_CENTER: Vec2<i32> = Vec2::new(SCREEN_PIXELS.x as i32 / 2, SCREEN_PIXELS.y as i32 / 2);

pub const BORDER_MARGIN: Size = 2;
pub const BOARD_POSITION: Vec2<i32> = Vec2::new(10, 10);
pub const BOARD_DIMENSIONS: Size2 = Vec2::new(10, 20);
