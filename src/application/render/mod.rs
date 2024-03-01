use std::rc::Rc;

use num::{Num, Unsigned};
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::application::asset::texture::{SrcRect, Texture};
use crate::application::geometry::{
  Cir2, IntConvertable, Line2, Pol2, Ray2, Rec2, SizePrimitive, UnitPrimitive, Vec2,
};
use crate::application::render::color::RGBA;

pub mod color;

#[derive(Clone)]
pub struct Properties {
  pub title: String,
  pub dimensions: Vec2<u32>,
  pub logical: Option<Vec2<u32>>,
  pub fullscreen: bool,
  pub show_cursor: bool,
  pub vsync: bool,
  pub opengl: bool,
  pub hardware_acceleration: bool,
  pub software_acceleration: bool,
  pub screen_color: RGBA,
}

pub struct Renderer {
  subsystem: sdl2::render::WindowCanvas,
  properties: Properties,
}

impl Renderer {
  pub fn new(context: &sdl2::Sdl, properties: Properties) -> Self {
    let window = new_window(context, properties.clone());

    // apply pre-construction properties
    let mut builder = window.into_canvas(); // takes ownership of `Window`
    if properties.vsync { builder = builder.present_vsync(); }
    if properties.hardware_acceleration { builder = builder.accelerated(); }
    if properties.software_acceleration { builder = builder.software(); }
    if !properties.show_cursor { context.mouse().show_cursor(false); }

    // build renderer subsystem
    let mut subsystem = builder
      .build()
      .map_err(|e| e.to_string())
      .unwrap();

    // apply post-construction properties
    if let Some(size) = properties.logical {
      subsystem.set_logical_size(size.x, size.y).map_err(|e| e.to_string()).unwrap();
    }

    subsystem.set_draw_color(properties.screen_color);

    Self {
      subsystem,
      properties,
    }
  }

  pub fn new_texture_creator(&self) -> TextureCreator<WindowContext> { self.subsystem.texture_creator() }

  pub fn present(&mut self) {
    self.subsystem.present();
    self.set_color(self.properties.screen_color);
    self.subsystem.clear();
  }

  fn set_color(&mut self, color: RGBA) {
    self.subsystem.set_draw_color(color);
  }

  pub fn draw_texture<T: IntConvertable>(&mut self, texture: &Rc<Texture>, position: Vec2<T>) {
    let (x, y) = position.destructure();
    let (w, h) = texture.dimensions.destructure();
    let src = Rect::new(0, 0, w, h);
    let dest = Rect::new(x.into(), y.into(), w, h);
    self.subsystem.copy(&texture.internal, src, dest).unwrap();
  }

  pub fn draw_from_texture<T: IntConvertable>(&mut self, texture: &Rc<Texture>, position: Vec2<T>, from: SrcRect) {
    let (x, y) = position.destructure();
    let ((sx, sy), (w, h)) = from.destructure();
    let dest = Rect::new(x.into(), y.into(), w, h);
    let src = Rect::new(sx as i32, sy as i32, w, h);
    self.subsystem.copy(&texture.internal, src, dest).unwrap();
  }

  pub fn draw_vec<T: IntConvertable>(&mut self, vec: Vec2<T>, color: RGBA) {
    self.set_color(color);
    self.subsystem.draw_point(Point::from(vec))
      .map_err(|error| eprintln!("{error}"))
      .ok();
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
    panic!("not implemented");
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

  pub fn draw_rect_filled<T: IntConvertable, U: SizePrimitive>(&mut self, rect: Rec2<T, U>, outline: RGBA, fill: RGBA) {
    let raw_rect = Rect::from(rect);

    // draw fill
    self.set_color(fill);
    self.subsystem
      .fill_rect(raw_rect)
      .map_err(|error| eprintln!("{error}"))
      .ok();

    // draw outline
    self.set_color(outline);
    self.subsystem
      .draw_rect(raw_rect)
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
    panic!("not implemented");
  }

  pub fn draw_circle<T: IntConvertable>(&mut self, circle: Cir2<T>, color: RGBA) {
    self.set_color(color);
    panic!("not implemented");
  }

  pub fn draw_circle_filled<T: IntConvertable>(&mut self, circle: Cir2<T>, outline: RGBA, fill: RGBA) {
    panic!("not implemented");
  }
}

/// Create a new `sdl2::video::Window` with the given `RendererProperties`
fn new_window(context: &sdl2::Sdl, properties: Properties) -> sdl2::video::Window {
  let (w, h) = properties.dimensions.destructure();
  let video_subsystem = context.video().unwrap();

  let mut builder = video_subsystem.window(properties.title.as_str(), w, h);
  if properties.fullscreen { builder.fullscreen_desktop(); };
  if properties.opengl { builder.opengl(); };

  let window = builder.build().map_err(|e| e.to_string()).unwrap();
  window
}
