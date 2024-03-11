use std::rc::Rc;

use sdl2::ttf::Font;

use crate::engine::asset::texture::{Texture, TextureLoader};
use crate::engine::render::color::RGBA;

pub struct Text {
  content: String,
  color: RGBA,
}

impl Text {
  pub fn new(content: String, color: RGBA) -> Self {
    Self {
      content,
      color,
    }
  }

  /// Builds a `Texture` from `content` in `font`
  pub fn build_texture<'t>(&self, font: &Rc<Font<'t, 't>>, texture_loader: &'t TextureLoader) -> Result<Texture, String> {
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
    Ok(())
  }
}