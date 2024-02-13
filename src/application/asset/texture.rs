use std::collections::HashMap;

use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, TextureQuery};
use sdl2::video::WindowContext;

use crate::application::geometry::Vec2;

/// Stores loaded textures by basename
pub struct TextureStore {
  textures: HashMap<String, Texture>,
}

impl TextureStore {
  pub fn new() -> Self { Self { textures: HashMap::new() } }
  pub fn add(&mut self, name: String, texture: Texture) -> &Texture { self.textures.entry(name).or_insert(texture) }
  pub fn get(&self, name: &str) -> Result<&Texture, &str> { self.textures.get(name).ok_or("Failed to get texture") }
}

/// Load textures from files into a store
pub struct TextureLoader {
  store: TextureStore,
  subsystem: TextureCreator<WindowContext>,
}

impl TextureLoader {
  pub fn new(creator: TextureCreator<WindowContext>) -> Self {
    let store = TextureStore::new();
    Self { subsystem: creator, store }
  }

  pub fn load(&mut self, filepath: String) -> Result<(), &str> {
    let internal_texture = self.subsystem.load_texture(filepath.as_str()).map_err(|_| "Failed to load texture")?;
    let texture = Texture::new(internal_texture);

    let filename = filepath.split("/").last().ok_or("Failed to get filename")?;
    let basename = filename.split(".").next().ok_or("Failed to get basename")?;

    self.store.add(String::from(basename), texture);
    Ok(())
  }

  pub fn use_store(&self) -> &TextureStore { &self.store }
}

/// A texture that can be drawn to the screen.
pub struct Texture {
  pub internal: sdl2::render::Texture,
  pub dimensions: Vec2<u32>,
}

impl Texture {
  pub fn new(texture: sdl2::render::Texture) -> Self {
    let TextureQuery { width, height, .. } = texture.query();
    let dimensions = Vec2::new(width, height);
    Self { internal: texture, dimensions }
  }
}
