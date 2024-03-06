use std::rc::Rc;

use crate::application::Application;
use crate::application::asset::audio::{Loop, SoundType};
use crate::application::event::EventStore;
use crate::application::geometry::Vec2;
use crate::application::manager::assets::{AssetManager, AssetType};
use crate::application::render::{Properties, Renderer};
use crate::application::tile::tileset::Tileset;
use crate::application::time::{ConsumeAction, Timer};
use crate::board::{Board, BoardEvent};
use crate::constants::board::TILE_SIZE;
use crate::constants::game::{CLEAR_COOLDOWN, LINES_PER_LEVEL, MAX_TETRIS_LEVEL, SPAWN_COOLDOWN, START_TETRIS_LEVEL};
use crate::constants::window::{SCREEN_COLOR, SCREEN_PIXELS, TITLE, WINDOW_DIMENSIONS};
use crate::math::{calculate_score, calculate_speed_ms, determine_sfx};

mod application;
mod constants;
mod piece;
mod board;
mod math;
mod algorithm;

struct Tetris {
  level: u32,
  score: u32,
  lines: u32,
  spawn_cooldown: Timer,
  drop_cooldown: Timer,
  lines_to_clear: Vec<usize>,
  board: Board,
}

pub fn main() -> Result<(), ()> {
  Application::new(Properties {
    title: String::from(TITLE),
    dimensions: WINDOW_DIMENSIONS,
    logical: Some(SCREEN_PIXELS),
    fullscreen: false,
    show_cursor: false,
    vsync: true,
    opengl: true,
    software_acceleration: false,
    hardware_acceleration: true,
    screen_color: SCREEN_COLOR,
  })
    .on_load_assets(handle_load_resources)
    .on_start(handle_start)
    .on_update(handle_update)
    .on_render(handle_render)
    .run()
}

fn handle_load_resources(assets: &mut AssetManager) {
  assets.load(AssetType::Texture, String::from("asset/spritesheet.png")).expect("failed to load texture");                                    // spritesheet

  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/level.ogg")).expect("failed to load sound effect");     // level advance
  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/line.ogg")).expect("failed to load sound effect");      // line clear
  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/tetris.ogg")).expect("failed to load sound effect");    // tetris clear
  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/shift.ogg")).expect("failed to load sound effect");     // line shift
  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/move.ogg")).expect("failed to load sound effect");      // piece transform
  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/rotate.ogg")).expect("failed to load sound effect");    // piece rotation
  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/land.ogg")).expect("failed to load sound effect");      // piece land
  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/pause.ogg")).expect("failed to load sound effect");     // game pause
  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/gameover.ogg")).expect("failed to load sound effect");  // game over
  assets.load(AssetType::Audio { sound_type: SoundType::Music }, String::from("asset/korobeiniki.ogg")).expect("failed to load music");       // tetris theme

  // create tileset
  let (textures, ..) = assets.use_store();
  let texture = textures.get("spritesheet").expect("failed to fetch texture for building assets");
  assets.tilesets.add(String::from("spritesheet"), Rc::new(Tileset::new(texture.clone(), Vec2::new(TILE_SIZE, TILE_SIZE))));
}

fn handle_start(assets: &AssetManager) -> Tetris {
  let tileset = assets.tilesets.get("spritesheet").expect("failed to fetch tileset");

  // create board
  let mut board = Board::new(tileset);

  // spawn first piece
  board.spawn_piece();

  // start music
  assets.audio.play("korobeiniki", 8, Loop::Forever).expect("failed to play music");

  Tetris {
    level: START_TETRIS_LEVEL,
    score: 0,
    lines: 0,
    spawn_cooldown: Timer::new(SPAWN_COOLDOWN, false),
    drop_cooldown: Timer::new(CLEAR_COOLDOWN, false),
    lines_to_clear: Vec::new(),
    board,
  }
}

fn handle_update(events: &EventStore, assets: &AssetManager, state: &mut Tetris, _: &mut Renderer) {
  match state.board.update(events) {
    BoardEvent::MoveLeft | BoardEvent::MoveRight => {
      // play sound effect
      assets.audio.play("move", 24, Loop::Once).expect("failed to play sound effect");
    }
    BoardEvent::Rotate => {
      // play sound effect
      assets.audio.play("rotate", 24, Loop::Once).expect("failed to play sound effect");
    }
    BoardEvent::Land => {
      // play sound effect
      assets.audio.play("land", 25, Loop::Once).expect("failed to play sound effect");

      // delete active piece
      state.board.kill_piece();

      // check for full lines
      state.lines_to_clear = state.board.get_full_lines();
      let lines_cleared = state.lines_to_clear.len() as u32;
      if lines_cleared > 0 {
        state.lines += lines_cleared;

        if let clear_line_sfx = determine_sfx(lines_cleared).expect("failed to determine sfx") {
          assets.audio.play(clear_line_sfx, 24, Loop::Once).expect("failed to play sound effect");
        }

        // clear lines
        for line in &state.lines_to_clear {
          state.board.clear_line(*line).expect("failed to clear line");
        }

        // todo: Well, We should check if there is anything too drop and skip the first cooldown
        // start the drop cooldown
        state.drop_cooldown.start();
      } else {
        // no lines to clear, start the spawn cooldown
        state.spawn_cooldown.start();
      }
    }
    _ => {}
  }

  // check if the drop cooldown is done
  if state.drop_cooldown.consume(ConsumeAction::Disable) {
    // get full lines
    let lines_cleared = state.lines_to_clear.len() as u32;
    if lines_cleared > 0 {
      // drop full lines
      for line in &state.lines_to_clear {
        state.board.move_lines_down(*line).expect("failed to clear line");
      }
      state.lines_to_clear.clear(); // done

      // calculate score
      let points = calculate_score(lines_cleared, state.level).expect("failed to calculate score");
      state.score += points;

      // check level advance
      if state.lines >= state.level * LINES_PER_LEVEL {
        state.level += 1;
        if (state.level <= MAX_TETRIS_LEVEL) {
          let new_speed = calculate_speed_ms(state.level).expect("failed to calculate speed");
          state.board.set_speed_ms(new_speed);
        } else {
          // todo: handle beat game, good luck testing this
        }
      }

      // print score
      println!("Level: {}, Score: {}, Lines: {}", state.level, state.score, state.lines);
    }

    // drop sfx
    assets.audio.play("shift", 24, Loop::Once).expect("failed to play sound effect");

    // start the spawn cooldown
    state.spawn_cooldown.start();
  }

  // check if the spawn cooldown is done
  if state.spawn_cooldown.consume(ConsumeAction::Disable) {
    state.board.spawn_piece();
  }
}

fn handle_render(state: &Tetris, renderer: &mut Renderer) {
  state.board.render(renderer);

  // todo: render score, level, lines, preview, etc.
}