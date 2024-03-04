use crate::application::geometry::Vec2;
use crate::application::render::color::{color, RGBA};

pub const TITLE: &str = "Tetris";
pub const SCREEN_COLOR: RGBA = color::MANTLE;
pub const WINDOW_DIMENSIONS: Vec2<u32> = Vec2::new(1000, 900);
pub const SCREEN_PIXELS: Vec2<u32> = Vec2::new(200, 180);
