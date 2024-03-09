use std::rc::Rc;

use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::engine::structure::store::HeapStore;

pub type TypefaceStore<'ttf, 'f> = HeapStore<Font<'ttf, 'f>>;

pub struct TypefaceLoader<'ttf, 'b> {
  subsystem: &'ttf Sdl2TtfContext,
  store: TypefaceStore<'ttf, 'b>,
}

impl<'ttf, 'l> TypefaceLoader<'ttf, 'l> {
  pub fn new(subsystem: &'ttf Sdl2TtfContext) -> Self {
    Self {
      subsystem,
      store: TypefaceStore::new(),
    }
  }

  pub fn load(&mut self, filepath: String, size: u16) -> Result<(), String> {
    let font = self.subsystem.load_font(filepath.as_str(), size)?;

    let filename = filepath.split("/").last().ok_or("Failed to get filename")?;
    let basename = filename.split(".").next().ok_or("Failed to get basename")?;

    self.store.add(String::from(basename), Rc::new(font));

    Ok(())
  }

  pub fn use_store(&self) -> &TypefaceStore {
    &self.store
  }
}