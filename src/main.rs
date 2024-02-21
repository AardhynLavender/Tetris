use std::rc::Rc;

use crate::application::Application;
use crate::application::asset::audio::{Loop, SoundType};
use crate::application::event::EventStore;
use crate::application::geometry::{Line2, Pol2, Rec2, Vec2};
use crate::application::manager::assets::{AssetManager, AssetType};
use crate::application::manager::sprite::{SpriteManager, SpriteType};
use crate::application::render::{Properties, Renderer};
use crate::application::render::color::color;
use crate::application::tile::tile::TileId;
use crate::application::tile::tilemap::Tilemap;
use crate::application::tile::tileset::Tileset;
use crate::application::utility::types::Size2;

mod application;

const SCREEN_PIXELS: Vec2<u32> = Vec2::new(192, 108);
const BOARD_DIMENSIONS: Size2 = Vec2::new(10, 20);
const BOARD_POSITION: Vec2<i32> = Vec2::new(8, 8);
const TILE_SIZE: Vec2<u32> = Vec2::new(8, 8);
const PURPLE_TILE: TileId = 0;
const BLUE_TILE: TileId = 2;

#[derive(Default)]
struct Tetris {
  level: u32,
  score: u32,
  lines: u32,
}

pub fn main() -> Result<(), ()> {
  Application::new(Properties {
    title: String::from("Tetris"),
    dimensions: Vec2 { x: 1920, y: 1080 },
    logical: Some(SCREEN_PIXELS),
    fullscreen: false,
    show_cursor: false,
    vsync: true,
    opengl: true,
    software_acceleration: false,
    hardware_acceleration: true,
  })
    .on_load_assets(handle_load_resources)
    .on_start(handle_start)
    .on_update(handle_update)
    .use_state(handle_state)
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
  assets.tilesets.add(String::from("spritesheet"), Rc::new(Tileset::new(texture, TILE_SIZE)));
}

fn handle_state() -> Tetris {
  Tetris { level: 0, score: 0, lines: 0 }
}

fn handle_start(assets: &AssetManager, sprites: &mut SpriteManager, _: &Tetris) {
  println!("Tetris started!");

  let tileset = assets.tilesets.get("spritesheet").expect("failed to fetch tileset");
  let mut tilemap = Tilemap::new(Rc::clone(&tileset), BOARD_POSITION, BOARD_DIMENSIONS);

  // checkerboard pattern
  for y in 0..BOARD_DIMENSIONS.y {
    for x in 0..BOARD_DIMENSIONS.x {
      let tile_id = if (x + y) % 2 == 0 { PURPLE_TILE } else { BLUE_TILE };
      tilemap.set_tile_at_coord(Vec2::new(x, y), tileset.get_tiledata(tile_id).expect("failed to fetch tiledata"));
    }
  }

  sprites.add(String::from("board"), SpriteType::Tilemap(tilemap));
}

fn handle_update(events: &EventStore, sprites: &SpriteManager, assets: &AssetManager, renderer: &mut Renderer) {
  let (textures, audio, ..) = assets.use_store();
  let mouse = events.get_mouse_position();

  // draw texture
  renderer.draw_texture(&textures.get("spritesheet").expect("failed to fetch texture for rendering"), Vec2::new(150, 10));

  // play music
  if events.is_pressed(sdl2::keyboard::Keycode::Space) {
    audio.play("korobeiniki", 10, Loop::Forever).unwrap();
  }

  // play sounds
  if events.is_pressed(sdl2::keyboard::Keycode::Return) {
    audio.play("tetris", 10, Loop::Once).unwrap();
  }

  // draw tilemap
  let tilemap = sprites.get("board").expect("failed to fetch tilemap for rendering");
  if let SpriteType::Tilemap(tilemap) = tilemap {
    for tile in tilemap {
      if let Some(tile) = tile {
        renderer.draw_from_texture(
          &textures.get("spritesheet").expect("failed to fetch texture for rendering"),
          tile.position,
          tile.src,
        );
      }
    }
  }

  // draw rectangle
  let rect_position = Vec2::new(mouse.x - 32, mouse.y - 32);
  let rect = Rec2::new(rect_position, Vec2::new(64u8, 64u8));
  renderer.draw_rect(rect, color::GREEN);

  // draw line
  let line = Line2::new(Vec2::new(SCREEN_PIXELS.x as i32, SCREEN_PIXELS.y as i32), mouse);
  renderer.draw_line(line, color::RED);

  // draw polygon
  let polygon = Pol2::new(vec![
    Vec2::new(8, 8),
    mouse,
    Vec2::new(8, (SCREEN_PIXELS.y as i32 - 8)),
  ]);
  renderer.draw_poly(polygon, color::CYAN);

  // draw vector
  renderer.draw_vec(mouse, color::MAGENTA);
}