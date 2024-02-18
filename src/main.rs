use crate::application::Application;
use crate::application::asset::audio::{Loop, SoundType};
use crate::application::events::EventStore;
use crate::application::geometry::{Line2, Pol2, Rec2, Vec2};
use crate::application::managers::{AssetManager, AssetType};
use crate::application::render::{Properties, Renderer};
use crate::application::render::color::color;
use crate::application::utility::types::Size2;

mod application;

const SCREEN_PIXELS: Vec2<u32> = Vec2::new(192, 108);
const BOARD_DIMENSIONS: Size2 = Vec2::new(10, 20);
const BOARD_POSITION: Vec2<i32> = Vec2::new(8, 8);
const TILE_SIZE: Vec2<u32> = Vec2::new(8, 8);

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
    .on_load_resources(handle_load_resources)
    .on_start(handle_start)
    .on_update(handle_update)
    .use_state(handle_state)
    .run()
}

fn handle_load_resources(assets: &mut AssetManager) {
  assets.load(AssetType::Texture, String::from("asset/spritesheet.png"))
    .expect("failed to load texture");
  assets.load(AssetType::Audio { sound_type: SoundType::Effect }, String::from("asset/tetris.ogg"))
    .expect("failed to load sound effect"); // clear 4 lines effect
  assets.load(AssetType::Audio { sound_type: SoundType::Music }, String::from("asset/korobeiniki.ogg"))
    .expect("failed to load music"); // tetris theme
}

fn handle_state() -> Tetris {
  Tetris { level: 0, score: 0, lines: 0 }
}

fn handle_start() {
  println!("Tetris started!");
}

fn handle_update(events: &EventStore, assets: &AssetManager, renderer: &mut Renderer) {
  let (textures, audio, ..) = assets.use_store();
  let mouse = events.get_mouse_position();

  // draw texture
  renderer.draw_texture(textures.get("spritesheet").expect("failed to fetch texture for rendering"), Vec2::new(150, 10));

  // play music
  if events.is_pressed(sdl2::keyboard::Keycode::Space) {
    audio.play("korobeiniki", 10, Loop::Forever).unwrap();
  }

  // play sounds
  if events.is_pressed(sdl2::keyboard::Keycode::Return) {
    audio.play("tetris", 10, Loop::Once).unwrap();
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