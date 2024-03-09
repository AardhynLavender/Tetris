use std::rc::Rc;

use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, TextureQuery};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use crate::application::geometry::{Rec2, Vec2};
use crate::application::structure::store::HeapStore;

pub type TextureStore = HeapStore<Texture>;

pub struct TextureLoader {
  store: TextureStore,
  subsystem: TextureCreator<WindowContext>,
}

impl TextureLoader {
  pub fn new(creator: TextureCreator<WindowContext>) -> Self {
    let store = HeapStore::new();
    Self { subsystem: creator, store }
  }

  /// Loads a texture from a file and adds it to the store
  pub fn load(&mut self, filepath: String) -> Result<String, &str> {
    let internal_texture = self.subsystem.load_texture(filepath.as_str()).map_err(|_| "Failed to load texture")?;
    let texture = Rc::new(Texture::new(internal_texture));

    let filename = filepath.split("/").last().ok_or("Failed to get filename")?;
    let basename = filename.split(".").next().ok_or("Failed to get basename")?;

    self.store.add(String::from(basename), texture);
    Ok(String::from(basename))
  }

  /// Adds a texture to the store
  pub fn add(&mut self, name: String, texture: Rc<Texture>) {
    self.store.add(name, texture);
  }

  /// Builds a texture from a surface
  pub fn build_from_surface(&self, surface: Surface) -> Result<Texture, &str> {
    let internal_texture = self.subsystem
      .create_texture_from_surface(surface)
      .map_err(|_| "Failed to load texture")?;
    let texture = Texture::new(internal_texture);
    Ok(texture)
  }

  /// Returns an immutable reference to the store
  pub fn use_store(&self) -> &TextureStore {
    &self.store
  }
}

pub type SrcRect = Rec2<u32, u32>;

pub struct Texture {
  pub internal: sdl2::render::Texture,
  pub dimensions: Vec2<u32>,
}

impl Texture {
  /// Create a new texture
  pub fn new(texture: sdl2::render::Texture) -> Self {
    let TextureQuery { width, height, .. } = texture.query();
    let dimensions = Vec2::new(width, height);
    Self { internal: texture, dimensions }
  }
}
