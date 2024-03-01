use std::rc::Rc;

use crate::application::Application;
use crate::application::asset::audio::SoundType;
use crate::application::event::EventStore;
use crate::application::geometry::Vec2;
use crate::application::manager::assets::{AssetManager, AssetType};
use crate::application::manager::object::{Object, ObjectManager};
use crate::application::render::{Properties, Renderer};
use crate::application::render::color::{color, RGBA};
use crate::application::tile::tileset::Tileset;
use crate::board::Board;

mod application;
mod constants;
mod piece;
mod board;

const SCREEN_COLOR: RGBA = color::MANTLE;
const WINDOW_DIMENSIONS: Vec2<u32> = Vec2::new(1920, 1080);
const SCREEN_PIXELS: Vec2<u32> = Vec2::new(384, 216);
const TILE_SIZE: Vec2<u32> = Vec2::new(8, 8);

struct Tetris {
  level: u32,
  score: u32,
  lines: u32,
  board: Board,
}

pub fn main() -> Result<(), ()> {
  Application::new(Properties {
    title: String::from("Tetris"),
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
  assets.tilesets.add(String::from("spritesheet"), Rc::new(Tileset::new(texture.clone(), TILE_SIZE)));
}

fn handle_start(assets: &AssetManager, _: &mut ObjectManager<Tetris>) -> Tetris {
  let tileset = assets.tilesets.get("spritesheet").expect("failed to fetch tileset");

  let mut board = Board::new(tileset);
  board.spawn_piece();

  Tetris {
    level: 0,
    score: 0,
    lines: 0,
    board,
  }
}

fn handle_update(events: &EventStore, _: &mut ObjectManager<Tetris>, state: &mut Tetris, _: &mut Renderer) {
  state.board.update(events);
}

fn handle_render(_: &ObjectManager<Tetris>, state: &Tetris, renderer: &mut Renderer) {
  state.board.render(renderer);
}