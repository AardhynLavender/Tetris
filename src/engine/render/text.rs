use std::rc::Rc;

use sdl2::ttf::Font;

use crate::engine::asset::texture::{Texture, TextureLoader};
use crate::engine::geometry::Vec2;
use crate::engine::render::color::RGBA;
use crate::engine::render::Renderer;

pub struct Text {
  content: String,
  dirty: bool,
  color: RGBA,
  position: Vec2<i32>,
  texture: Option<Rc::<Texture>>,
}

impl Text {
  pub fn new(content: String, color: RGBA, position: Vec2<i32>) -> Self {
    Self {
      content,
      position,
      dirty: true,
      texture: None,
      color,
    }
  }

  /// Builds a `Texture` from `content` in `font`
  fn build_texture<'t>(&self, font: &Rc<Font<'t, 't>>, texture_loader: &'t TextureLoader) -> Result<Texture, String> {
    let surface = font
      .render(self.content.as_str())
      .blended(self.color)
      .map_err(|e| e.to_string())?;
    let texture = texture_loader.build_from_surface(surface)?;
    Ok(texture)
  }

  /// Sets the text content of the text and
  pub fn set_content(&mut self, content: String) -> Result<(), String> {
    self.content = content;
    self.dirty = true;
    Ok(())
  }

  pub fn render<'t>(&mut self, font: &Rc<Font<'t, 't>>, texture_loader: &'t TextureLoader, renderer: &mut Renderer) {
    if self.dirty {
      self.texture = None;
    }

    if self.texture.is_none() {
      match self.build_texture(font, texture_loader) {
        Ok(texture) => {
          self.texture = Some(Rc::new(texture));
          self.dirty = false;
        }
        Err(e) => {
          eprintln!("Failed to build texture: {}", e);
        }
      }
    }

    if let Some(texture) = &self.texture {
      renderer.draw_texture(&texture, self.position);
    }
  }
}