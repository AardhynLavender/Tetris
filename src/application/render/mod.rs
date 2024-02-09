use num::{Num, Unsigned};
use sdl2::rect::Rect;

use crate::application::geometry::{
  Cir2, IntConvertable, Line2, Pol2, Ray2, Rec2, SizePrimitive, UnitPrimitive, Vec2,
};
use crate::application::render::color::RGBA;

pub mod color;

#[derive(Clone)]
pub struct RendererProperties {
  pub title: String,
  pub dimensions: Vec2<u32>,
  pub fullscreen: bool,
  pub show_cursor: bool,
  pub opengl: bool,
}

pub struct Renderer {
  subsystem: sdl2::render::WindowCanvas,
  properties: RendererProperties,
}

impl Renderer {
  pub fn new(context: &sdl2::Sdl, properties: RendererProperties) -> Self {
    let window = new_window(context, properties.clone());
    let subsystem = window
      .into_canvas() // takes ownership of `Window`
      .build()
      .map_err(|e| e.to_string())
      .unwrap();

    Self {
      subsystem,
      properties,
    }
  }
}

impl Renderer {
  pub fn present(&mut self) {
    self.subsystem.present();
    self.set_color(RGBA::default());
    self.subsystem.clear();
  }

  fn set_color(&mut self, color: RGBA) {
    self.subsystem.set_draw_color(color);
  }

  pub fn draw_vec<T: IntConvertable>(&mut self, vec: Vec2<T>, color: RGBA) {
    self.set_color(color);
    //self.subsystem.draw_point(vec.into())
    //  .map_err(|error| eprintln!("{error}"))
    //  .ok();
  }

  pub fn draw_from<T: IntConvertable>(&mut self, from: Vec2<T>, to: Vec2<T>, color: RGBA) {
    self.set_color(color);
    self.subsystem
      .draw_line(from, to)
      .map_err(|error| eprintln!("{error}"))
      .ok();
  }

  pub fn draw_line<T: IntConvertable>(&mut self, line: Line2<T>, color: RGBA) {
    self.draw_from(line.start, line.end, color);
  }

  pub fn draw_ray<T: IntConvertable>(&mut self, ray: Ray2<T>, color: RGBA, max_length: T) {
    self.set_color(color);
    // todo: implement
  }

  pub fn draw_rect<T: IntConvertable, U: SizePrimitive>(
    &mut self,
    rect: Rec2<T, U>,
    color: RGBA,
  ) {
    self.set_color(color);
    self.subsystem
      .draw_rect(Rect::from(rect))
      .map_err(|error| eprintln!("{error}"))
      .ok();
  }

  pub fn draw_rect_filled<T: IntConvertable, U: SizePrimitive>(
    &mut self,
    rect: Rec2<T, U>,
    outline: RGBA,
    fill: RGBA,
  ) {
    let raw_rect = Rect::from(rect);

    // draw fill
    self.set_color(fill);
    self.subsystem
      .draw_rect(raw_rect)
      .map_err(|error| eprintln!("{error}"))
      .ok();

    // draw outline
    self.set_color(outline);
    self.subsystem
      .fill_rect(raw_rect)
      .map_err(|error| eprintln!("{error}"))
      .ok();
  }

  pub fn draw_poly<T: IntConvertable>(&mut self, pol: Pol2<T>, color: RGBA) {
    let vertices = pol.vertices.len();
    for (i, from) in pol.vertices.iter().enumerate() {
      let to = &pol.vertices[(i + 1) % vertices];
      self.draw_from(*from, *to, color);
    }
  }

  pub fn draw_poly_filled<T: IntConvertable>(&mut self, pol: Pol2<T>, outline: RGBA, fill: RGBA) {
    self.set_color(outline);
    // todo: implement
  }

  pub fn draw_circle<T: IntConvertable>(&mut self, circle: Cir2<T>, color: RGBA) {
    self.set_color(color);
    // todo: implement
  }

  pub fn draw_circle_filled<T: IntConvertable>(
    &mut self,
    circle: Cir2<T>,
    outline: RGBA,
    fill: RGBA,
  ) {
    self.set_color(outline);
    // todo: implement
  }
}

fn new_window(subsystem: &sdl2::Sdl, properties: RendererProperties) -> sdl2::video::Window {
  let (w, h) = properties.dimensions.destructure();
  let video_subsystem = subsystem.video().unwrap();
  let builder = video_subsystem.window(properties.title.as_str(), w, h);

  let window = builder.build().map_err(|e| e.to_string()).unwrap();

  window
}
