use sdl2::ttf::Sdl2TtfContext;

use crate::application::asset::audio::{AudioPlayer, SoundType};
use crate::application::asset::texture::{TextureLoader, TextureStore};
use crate::application::asset::typography::TypefaceLoader;
use crate::application::tile::tileset::TilesetStore;

// Static immutable game assets //

pub enum AssetType {
  Texture,
  Audio { sound_type: SoundType },
  Typeface { font_size: u16 },
}

pub struct AssetManager<'ttf> {
  pub textures: TextureLoader,
  pub audio: AudioPlayer,
  pub tilesets: TilesetStore,
  pub typefaces: TypefaceLoader<'ttf, 'ttf>,
}

impl<'ttf> AssetManager<'ttf> {
  pub fn new(textures: TextureLoader, audio: AudioPlayer, ttf_context: &'ttf Sdl2TtfContext) -> Self {
    Self {
      textures,
      audio,
      tilesets: TilesetStore::new(),
      typefaces: TypefaceLoader::new(ttf_context),
    }
  }

  pub fn load(&mut self, asset_type: AssetType, filepath: String) -> Result<(), &str> {
    match asset_type {
      AssetType::Texture => self.textures.load(filepath).map(|_| ()).map_err(|_| "Failed to load texture"),
      AssetType::Audio { sound_type } => self.audio.load(sound_type, filepath),
      AssetType::Typeface { font_size } => self.typefaces.load(filepath, font_size).map_err(|e| "Failed to load typeface")
    }
  }

  pub fn use_store(&mut self) -> (&TextureStore, &AudioPlayer, &mut TilesetStore) {
    (&self.textures.use_store(), &self.audio, &mut self.tilesets)
  }
}

// Runtime mutable game assets //
