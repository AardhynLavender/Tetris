use crate::engine::geometry::Vec2;
use crate::engine::render::color::{color, RGBA};

/**
 * Constants relating to the window and application
 */

pub const TITLE: &str = "Tetris";
pub const SCREEN_COLOR: RGBA = color::MANTLE;
pub const WINDOW_DIMENSIONS: Vec2<u32> = Vec2::new(950, 900);
pub const SCREEN_PIXELS: Vec2<u32> = Vec2::new(190, 180);
