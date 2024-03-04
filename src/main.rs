use std::rc::Rc;

use crate::application::Application;
use crate::application::asset::audio::SoundType;
use crate::application::event::EventStore;
use crate::application::geometry::Vec2;
use crate::application::manager::assets::{AssetManager, AssetType};
use crate::application::render::{Properties, Renderer};
use crate::application::tile::tileset::Tileset;
use crate::board::Board;
use crate::constants::board::TILE_SIZE;
use crate::constants::game::{LINES_PER_LEVEL, MAX_TETRIS_LEVEL, START_TETRIS_LEVEL};
use crate::constants::window::{SCREEN_COLOR, SCREEN_PIXELS, TITLE, WINDOW_DIMENSIONS};
use crate::math::{calculate_score, calculate_speed_ms};

mod application;
mod constants;
mod piece;
mod board;
mod math;

struct Tetris {
  level: u32,
  score: u32,
  lines: u32,
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
  // load textures
  assets.load(AssetType::Texture, String::from("asset/spritesheet.png"))
    .expect("failed to load texture");

  // load sound effects
  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/tetris.ogg"))
    .expect("failed to load sound effect"); // clear 4 lines effect

  // load music
  assets.load(AssetType::Audio { sound_type: SoundType::Music }, String::from("asset/korobeiniki.ogg"))
    .expect("failed to load music"); // tetris theme

  // create tileset
  let (textures, ..) = assets.use_store();
  let texture = textures.get("spritesheet").expect("failed to fetch texture for building assets");
  assets.tilesets.add(String::from("spritesheet"), Rc::new(Tileset::new(texture.clone(), Vec2::new(TILE_SIZE, TILE_SIZE))));
}

fn handle_start(assets: &AssetManager) -> Tetris {
  let tileset = assets.tilesets.get("spritesheet").expect("failed to fetch tileset");

  let mut board = Board::new(tileset);
  board.spawn_piece();

  Tetris {
    level: START_TETRIS_LEVEL,
    score: 0,
    lines: 0,
    board,
  }
}

fn handle_update(events: &EventStore, state: &mut Tetris, _: &mut Renderer) {
  state.board.update(events);

  if !state.board.drop_complete() {
    return;
  }

  // check for full lines
  let full_lines = state.board.get_full_lines();
  let lines_cleared = full_lines.len() as u32;

  if lines_cleared > 0 {
    state.lines += lines_cleared;

    // clear lines and drop
    for line in full_lines {
      state.board.clear_line(line).expect("failed to clear line");
      state.board.move_lines_down(line).expect("failed to move lines down");
    }

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

    println!("Level: {}, Score: {}, Lines: {}", state.level, state.score, state.lines);
  }

  state.board.spawn_piece();
}

fn handle_render(state: &Tetris, renderer: &mut Renderer) {
  state.board.render(renderer);

  // todo: render score, level, lines, preview, etc.
}