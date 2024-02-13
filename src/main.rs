use crate::application::Application;
use crate::application::asset::audio::{AudioLoader, AudioStore, Loop, SoundType};
use crate::application::asset::texture::{TextureLoader, TextureStore};
use crate::application::events::EventStore;
use crate::application::geometry::{Line2, Pol2, Rec2, Vec2};
use crate::application::render::{Properties, Renderer};
use crate::application::render::color::color;

mod application;

const SCREEN_PIXELS: Vec2<u32> = Vec2 { x: 192u32, y: 108u32 };

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
    .on_update(handle_update)
    .run()
}

fn handle_load_resources(texture: &mut TextureLoader, audio: &mut AudioLoader) {
  texture.load(String::from("asset/spritesheet.png")).unwrap();
  audio.load(SoundType::Effect, String::from("asset/tetris.ogg")).unwrap(); // clear level
  audio.load(SoundType::Music, String::from("asset/korobeiniki.ogg")).unwrap(); // tetris theme (A-type)
}

fn handle_update(events: &EventStore, textures: &TextureStore, audio: &AudioStore, renderer: &mut Renderer) {
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
  let rect = Rec2::new(Vec2::new(mouse.x - 32, mouse.y - 32), Vec2::new(64u8, 64u8));
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