use crate::constants::game::{DOUBLE_LINE_MULTIPLIER, FALL_COOLDOWN, LEVEL_SPEED_MULTIPLIER, MAX_LINES, MAX_TETRIS_LEVEL, SINGLE_LINE_MULTIPLIER, START_TETRIS_LEVEL, TETRIS_MULTIPLIER, TRIPLE_LINE_MULTIPLIER};

pub fn level_invariant(level: u32) -> Result<(), String> {
  if level < START_TETRIS_LEVEL {
    return Err(String::from("Level must be at least 1"));
  }
  if level > MAX_TETRIS_LEVEL {
    return Err(String::from("Level must be at most 29"));
  }
  Ok(())
}

pub fn calculate_score(lines: u32, level: u32) -> Result<u32, String> {
  level_invariant(level)?;

  if lines == 0 {
    return Ok(0);
  }

  let score = match lines {
    1 => SINGLE_LINE_MULTIPLIER,
    2 => DOUBLE_LINE_MULTIPLIER,
    3 => TRIPLE_LINE_MULTIPLIER,
    4 => TETRIS_MULTIPLIER,
    _ => return Err(String::from("Invalid number of lines cleared"))
  };

  Ok(score * level)
}

pub fn calculate_speed_ms(level: u32) -> Result<u64, String> {
  level_invariant(level)?;
  let speed = (FALL_COOLDOWN.as_millis() as f32 * (LEVEL_SPEED_MULTIPLIER.powf(level as f32))) as u64;
  Ok(speed)
}

pub fn determine_sfx(lines: u32) -> Option<&'static str> {
  if lines > 0 && lines < MAX_LINES {
    return Some("line");
  } else if lines == MAX_LINES {
    return Some("tetris");
  }
  return None;
}