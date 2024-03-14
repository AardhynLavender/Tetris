use std::time::Duration;

use crate::engine::geometry::{Rec2, Vec2};
use crate::engine::render::color::{color, RGBA};
use crate::engine::utility::types::{Size, Size2};

/**
 * Constants relating to the game
 */

// Displays //

pub const STATISTICS_BORDER: Rec2<i32, Size> = Rec2::new(Vec2::new(99, 8), Vec2::new(83u32, 31u32));
pub const SCORE_TEXT_POSITION: Vec2<i32> = Vec2::new(102, 10);
pub const LINES_TEXT_POSITION: Vec2<i32> = Vec2::new(102, 20);
pub const LEVEL_TEXT_POSITION: Vec2<i32> = Vec2::new(102, 30);

pub const PREVIEW_BORDER: Rec2<i32, Size> = Rec2::new(Vec2::new(99, 48), Vec2::new(83u32, 33u32));
pub const PREVIEW_POSITION: Vec2<i32> = Vec2::new(142, 49);
pub const PREVIEW_DIMENSIONS: Vec2<Size> = Vec2::new(4, 4);
pub const NEXT_TEXT_POSITION: Vec2<i32> = Vec2::new(108, 61);

// Sound //

pub const SFX_VOLUME: i32 = 24;
pub const MUSIC_VOLUME: i32 = 8;

// Board //

pub const TILE_SIZE: Size = 8;
pub const TILE_PIECE_MARGIN: Size = 1; // margin between pieces in the tileset

pub const BORDER_MARGIN: Size = 2;
pub const BORDER_COLOR: RGBA = color::SURFACE_0;
pub const BOARD_POSITION: Vec2<i32> = Vec2::new(10, 10);
pub const BOARD_DIMENSIONS: Size2 = Vec2::new(10, 20);

pub const FIRST_ROW: i32 = 0;
pub const SPAWN_OFFSET_X: i32 = 4; // center the piece on the board.rs

// Cooldowns //

pub const SPAWN_COOLDOWN: Duration = Duration::from_millis(300);
pub const CLEAR_COOLDOWN: Duration = Duration::from_millis(1_000);
pub const FALL_COOLDOWN: Duration = Duration::from_millis(1_000);
pub const PLAYER_DROP_COOLDOWN: Duration = Duration::from_millis(40);
pub const PLAYER_SLIDE_COOLDOWN: Duration = Duration::from_millis(100);

// Multipliers //

pub const LEVEL_SPEED_MULTIPLIER: f32 = 0.8;
pub const SINGLE_LINE_MULTIPLIER: u32 = 40;
pub const DOUBLE_LINE_MULTIPLIER: u32 = 100;
pub const TRIPLE_LINE_MULTIPLIER: u32 = 300;
pub const TETRIS_MULTIPLIER: u32 = 1200;

// Levels //

pub const START_TETRIS_LEVEL: u32 = 1;
pub const MAX_TETRIS_LEVEL: u32 = 29;

// Lines //

pub const LINES_PER_LEVEL: u32 = 10;
pub const MAX_LINES: u32 = 4;