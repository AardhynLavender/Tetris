use crate::application::asset::audio::{AudioLoader, AudioStore, SoundType};
use crate::application::asset::texture::{TextureLoader, TextureStore};

// Manage static immutable game assets
pub enum AssetType {
  Texture,
  Audio { sound_type: SoundType },
}

pub struct AssetManager {
  pub textures: TextureLoader,
  pub audio: AudioLoader,
}

impl AssetManager {
  pub fn new(textures: TextureLoader, audio: AudioLoader) -> Self {
    Self { textures, audio }
  }

  pub fn load(&mut self, asset_type: AssetType, filepath: String) -> Result<(), &str> {
    match asset_type {
      AssetType::Texture => self.textures.load(filepath).map(|_| ()).map_err(|_| "Failed to load texture"),
      AssetType::Audio { sound_type } => self.audio.load(sound_type, filepath),
    }
  }

  pub fn use_store(&self) -> (&TextureStore, &AudioStore) {
    (&self.textures.use_store(), &self.audio.use_store())
  }
}
