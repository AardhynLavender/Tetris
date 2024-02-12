use crate::application::Application;
use crate::application::asset::texture::{TextureLoader, TextureStore};
use crate::application::events::EventStore;
use crate::application::geometry::{Line2, Pol2, Rec2, Vec2};
use crate::application::render::{Properties, Renderer};
use crate::application::render::color::color;

mod application;

pub fn main() -> Result<(), ()> {
  Application::new(Properties {
    title: String::from("Tetris"),
    dimensions: Vec2 { x: 1920, y: 1080 },
    logical: Some(Vec2 { x: 96u32, y: 54u32 }),
    fullscreen: false,
    show_cursor: false,
    vsync: true,
    opengl: true,
    hardware_acceleration: true,
    software_acceleration: false,
  })
    .on_load_resources(handle_load_resources)
    .on_update(handle_update)
    .run()
}

fn handle_load_resources(loader: &mut TextureLoader) {
  loader.load(String::from("asset/spritesheet.png")).unwrap();
}

fn handle_update(events: &EventStore, textures: &TextureStore, renderer: &mut Renderer) {
  let mouse = events.get_mouse_position();

  // draw texture
  renderer.draw_texture(textures.get("spritesheet").expect("bad"), Vec2::new(10, 10));

  // draw rectangle
  let rect = Rec2::new(mouse, Vec2::new(100u8, 200u8));
  renderer.draw_rect(rect, color::GREEN);

  // draw line
  let line = Line2::new(Vec2::default(), mouse);
  renderer.draw_line(line, color::RED);

  // draw polygon
  let polygon = Pol2::new(vec![
    Vec2::new(100, 100),
    Vec2::new(200, 100),
    Vec2::new(100, 200),
  ]);
  renderer.draw_poly(polygon, color::CYAN);

  // draw vector
  renderer.draw_vec(mouse, color::MAGENTA);
}