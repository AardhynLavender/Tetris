use std::rc::Rc;

use sdl2::keyboard::Keycode;

use crate::algorithm::{calculate_score, calculate_speed_ms, determine_sfx};
use crate::board::{Board, BoardEvent, BoardState};
use crate::constants::game::{BORDER_COLOR, CLEAR_COOLDOWN, GAME_OVER_TEXT, GAME_PAUSED_TEXT, GAME_WON_TEXT, LEVEL_TEXT_POSITION, LINES_PER_LEVEL, LINES_TEXT_POSITION, MAX_TETRIS_LEVEL, MUSIC_VOLUME, NEXT_TEXT_POSITION, PREVIEW_BORDER, PREVIEW_DIMENSIONS, PREVIEW_POSITION, SCORE_TEXT_POSITION, SFX_VOLUME, SPAWN_COOLDOWN, START_TETRIS_LEVEL, STATE_TEXT_POSITION, STATISTICS_BORDER, TILE_SIZE};
use crate::constants::window::{SCREEN_COLOR, SCREEN_PIXELS, TITLE, WINDOW_DIMENSIONS};
use crate::engine::application::{Actions, run_application};
use crate::engine::asset::{AssetManager, AssetType};
use crate::engine::asset::audio::Loop;
use crate::engine::asset::audio::SoundType;
use crate::engine::event::EventStore;
use crate::engine::geometry::Vec2;
use crate::engine::render::{Properties, Renderer};
use crate::engine::render::color::color;
use crate::engine::render::text::Text;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::tile::tileset::Tileset;
use crate::engine::time::{ConsumeAction, Timer};
use crate::piece::{Piece, write_piece};

/**
 * Asset loading, main loop, and state management for the game.
 */

mod engine;
mod constants;
mod piece;
mod board;
mod algorithm;

#[derive(PartialEq, Clone)]
enum GameState {
  Playing,
  GameOver,
  Won,
  Pause,
}

// state
struct Tetris {
  board: Board,
  game_state: GameState,

  preview: Tilemap,
  next_text: Text,
  state_text: Text,

  level: u32,
  score: u32,
  lines: u32,
  score_text: Text,
  lines_text: Text,
  level_text: Text,

  spawn_cooldown: Timer,
  drop_cooldown: Timer,

  lines_to_clear: Vec<usize>,
}

// Initialization //

fn load(assets: &mut AssetManager) {
  assets.load(AssetType::Texture, String::from("asset/spritesheet.png")).expect("failed to load texture");                                    // spritesheet

  assets.load(AssetType::Typeface { font_size: 5 }, String::from("asset/typeface.ttf")).expect("failed to load typeface");                    // pixel font

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
  let (textures, _, tilesets) = assets.use_store();
  let texture = textures.get("spritesheet").expect("failed to fetch texture for building assets");
  tilesets.add(String::from("spritesheet"), Rc::new(Tileset::new(texture.clone(), Vec2::new(TILE_SIZE, TILE_SIZE))));
}

fn setup(assets: &AssetManager) -> Tetris {
  let tileset = assets.tilesets.get("spritesheet").expect("failed to fetch tileset");

  // create board
  let mut board = Board::new(Rc::clone(&tileset));

  // create preview
  let mut preview_board = Tilemap::new(tileset, PREVIEW_POSITION, PREVIEW_DIMENSIONS);

  // write preview
  let BoardState { preview, .. } = board.next_piece();
  write_preview(&mut preview_board, preview);

  // play music
  assets.audio.play("korobeiniki", MUSIC_VOLUME, Loop::Forever).expect("failed to play music");

  Tetris {
    game_state: GameState::Playing,
    board,
    preview: preview_board,

    spawn_cooldown: Timer::new(SPAWN_COOLDOWN, false),
    drop_cooldown: Timer::new(CLEAR_COOLDOWN, false),
    lines_to_clear: Vec::new(),

    level: START_TETRIS_LEVEL,
    score: 0,
    lines: 0,
    score_text: Text::new(String::from("score 0000000"), color::TEXT, SCORE_TEXT_POSITION),
    lines_text: Text::new(String::from("lines 0000000"), color::TEXT, LINES_TEXT_POSITION),
    level_text: Text::new(String::from(format!("level {:0>7}", START_TETRIS_LEVEL)), color::TEXT, LEVEL_TEXT_POSITION),
    next_text: Text::new(String::from("next"), color::TEXT, NEXT_TEXT_POSITION),
    state_text: Text::new(String::from(""), color::TEXT, STATE_TEXT_POSITION),
  }
}

// Rendering //

fn render(state: &mut Tetris, assets: &AssetManager, renderer: &mut Renderer) {
  state.board.render(renderer, state.game_state == GameState::Playing);

  state.state_text.render(&assets.typefaces.use_store().get("typeface").expect("failed to fetch typeface"), &assets.textures, renderer);

  render_preview(&state.preview, assets, &mut state.next_text, renderer);
  render_statistics(state, assets, renderer);
}

fn render_preview(preview: &Tilemap, assets: &AssetManager, text: &mut Text, renderer: &mut Renderer) {
  // draw preview
  for tile in preview {
    if let Some(tile) = tile {
      let position = Vec2::new(tile.position.x, tile.position.y);
      renderer.draw_from_texture(&preview.tileset.texture, position, tile.src);
    }
  }

  // draw border
  renderer.draw_rect(PREVIEW_BORDER, BORDER_COLOR);

  // draw text
  let typeface = assets.typefaces
    .use_store()
    .get("typeface")
    .expect("failed to fetch typeface");
  text.render(&typeface, &assets.textures, renderer);
}

fn render_statistics(state: &mut Tetris, assets: &AssetManager, renderer: &mut Renderer) {
  let typeface = assets.typefaces
    .use_store()
    .get("typeface")
    .expect("failed to fetch typeface");

  state.level_text.render(&typeface, &assets.textures, renderer);
  state.score_text.render(&typeface, &assets.textures, renderer);
  state.lines_text.render(&typeface, &assets.textures, renderer);

  renderer.draw_rect(STATISTICS_BORDER, BORDER_COLOR);
}

// Update //

fn update(events: &EventStore, assets: &AssetManager, state: &mut Tetris, renderer: &mut Renderer) {
  // check for pause
  if events.is_key_pressed(Keycode::Escape) {
    match state.game_state {
      GameState::Playing => {
        pause_sound(assets);
        state.state_text.set_content(String::from(GAME_PAUSED_TEXT));
        state.game_state = GameState::Pause;
      }
      GameState::Pause => {
        pause_sound(assets);
        state.state_text.clear_content();
        state.game_state = GameState::Playing;
      }
      _ => {}
    };
  }

  // check for fullscreen
  if events.is_key_pressed(Keycode::F11) {
    renderer.set_fullscreen(!renderer.is_fullscreen());
  }

  match &state.game_state {
    GameState::Playing => {
      match state.board.update(events) {
        BoardEvent::MoveLeft | BoardEvent::MoveRight => {
          // play sound effect
          assets.audio.play("move", SFX_VOLUME, Loop::Once).expect("failed to play sound effect");
        }
        BoardEvent::Rotate => {
          // play sound effect
          assets.audio.play("rotate", SFX_VOLUME, Loop::Once).expect("failed to play sound effect");
        }
        BoardEvent::Land => {
          // play sound effect
          assets.audio.play("land", SFX_VOLUME, Loop::Once).expect("failed to play sound effect");

          // delete active piece
          state.board.kill_piece();

          // check for full lines
          state.lines_to_clear = state.board.get_full_lines();
          let lines_cleared = state.lines_to_clear.len() as u32;
          if lines_cleared > 0 {
            state.lines += lines_cleared;
            state.lines_text.set_content(format!("LINES {:0>7}", state.lines));

            if let Some(clear_line_sfx) = determine_sfx(lines_cleared) {
              assets.audio.play(clear_line_sfx, SFX_VOLUME, Loop::Once).expect("failed to play sound effect");
            }

            // clear lines
            for line in &state.lines_to_clear {
              state.board.clear_line(*line).expect("failed to clear line");
            }

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
          state.score_text.set_content(format!("SCORE {:0>7}", state.score));

          // check level advance
          if state.lines >= state.level * LINES_PER_LEVEL {
            state.level += 1;
            state.level_text.set_content(format!("LEVEL {:0>7}", state.level));

            if state.level <= MAX_TETRIS_LEVEL {
              let new_speed = calculate_speed_ms(state.level).expect("failed to calculate speed");
              state.board.set_speed_ms(new_speed);
              assets.audio.play("level", SFX_VOLUME, Loop::Once).expect("failed to play sound effect");
            } else {
              state.game_state = GameState::Won;
            }
          }
        }

        // drop sfx
        assets.audio.play("shift", SFX_VOLUME, Loop::Once).expect("failed to play sound effect");

        // start the spawn cooldown
        state.spawn_cooldown.start();
      }

      // check if the spawn cooldown is done
      if state.spawn_cooldown.consume(ConsumeAction::Disable) {
        state.game_state = next_state(&mut state.board, &mut state.preview);
        if state.game_state == GameState::GameOver {
          assets.audio.stop("korobeiniki").expect("failed to stop music");
          assets.audio.play("gameover", SFX_VOLUME, Loop::Once).expect("failed to play sound effect");
        }
      }
    }
    GameState::Won => {
      state.state_text.set_content(String::from(GAME_WON_TEXT));
    }
    GameState::GameOver => {
      state.state_text.set_content(String::from(GAME_OVER_TEXT));
    }
    _ => {}
  }
}

fn write_preview(preview: &mut Tilemap, piece: &Piece) {
  preview.clear_tiles();
  write_piece(piece, preview);
}

fn next_state(board: &mut Board, preview_board: &mut Tilemap) -> GameState {
  let BoardState { preview, space, .. } = board.next_piece();
  write_preview(preview_board, preview);
  if space {
    GameState::Playing
  } else {
    GameState::GameOver
  }
}

fn pause_sound(assets: &AssetManager) {
  assets.audio.play("pause", SFX_VOLUME, Loop::Once).expect("failed to play sound effect");
}

// Main //

pub fn main() -> Result<(), String> {
  run_application(
    Properties {
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
    }, Actions {
      load,
      render,
      update,
      setup,
    })
}
