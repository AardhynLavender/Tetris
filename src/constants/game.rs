use std::time::Duration;

use crate::engine::geometry::{Rec2, Vec2};
use crate::engine::utility::types::Size;

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

// Cooldowns //

/// time it takes for a piece to spawn
pub const SPAWN_COOLDOWN: Duration = Duration::from_millis(300);
/// time it takes for lines to clear
pub const CLEAR_COOLDOWN: Duration = Duration::from_millis(1_000);
///Computer moves the piece down
pub const FALL_COOLDOWN: Duration = Duration::from_millis(1_000);
/// The Player moves the piece left or right
pub const PLAYER_DROP_COOLDOWN: Duration = Duration::from_millis(40);
/// The Player moves the piece down
pub const PLAYER_SLIDE_COOLDOWN: Duration = Duration::from_millis(100);

// Multipliers //

/// speed multiplier for each level
pub const LEVEL_SPEED_MULTIPLIER: f32 = 0.8;
/// point multiplier for one line cleared at level 1
pub const SINGLE_LINE_MULTIPLIER: u32 = 40;
/// point multiplier for two lines cleared at level 1
pub const DOUBLE_LINE_MULTIPLIER: u32 = 100;
/// point multiplier for three lines cleared at level 1
pub const TRIPLE_LINE_MULTIPLIER: u32 = 300;
/// point multiplier for four lines cleared at level 1
pub const TETRIS_MULTIPLIER: u32 = 1200;

// Levels //

/// level the player starts on
pub const START_TETRIS_LEVEL: u32 = 1;
/// the maximum level the player can reach
pub const MAX_TETRIS_LEVEL: u32 = 29;

// Lines //

/// the number of lines the player must clear to reach the next level
pub const LINES_PER_LEVEL: u32 = 10;
/// max number of lines that can be cleared at once
pub const MAX_LINES: u32 = 4;