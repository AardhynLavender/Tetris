use std::time::Duration;

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