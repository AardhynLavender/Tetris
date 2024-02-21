use crate::application::asset::audio::{AudioPlayer, SoundType};
use crate::application::asset::texture::{TextureLoader, TextureStore};
use crate::application::tile::tileset::TilesetStore;

// Static immutable game assets //

pub enum AssetType {
  Texture,
  Audio { sound_type: SoundType },
}

pub struct AssetManager {
  pub textures: TextureLoader,
  pub audio: AudioPlayer,
  pub tilesets: TilesetStore,
}

impl AssetManager {
  pub fn new(textures: TextureLoader, audio: AudioPlayer) -> Self {
    Self {
      textures,
      audio,
      tilesets: TilesetStore::new(),
    }
  }

  pub fn load(&mut self, asset_type: AssetType, filepath: String) -> Result<(), &str> {
    match asset_type {
      AssetType::Texture => self.textures.load(filepath).map(|_| ()).map_err(|_| "Failed to load texture"),
      AssetType::Audio { sound_type } => self.audio.load(sound_type, filepath),
    }
  }

  pub fn use_store(&self) -> (&TextureStore, &AudioPlayer, &TilesetStore) {
    (&self.textures.use_store(), &self.audio, &self.tilesets)
  }
}

// Runtime mutable game assets //
