use std::time::Duration;

///Computer moves the piece down
pub const FALL_COOLDOWN: Duration = Duration::from_millis(1000);
/// Player moves the piece
pub const PLAYER_TRANSFORM_COOLDOWN: Duration = Duration::from_millis(100);