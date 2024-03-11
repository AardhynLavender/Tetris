use sdl2::ttf::Sdl2TtfContext;

use crate::engine::asset::audio::{AudioPlayer, SoundType};
use crate::engine::asset::texture::{TextureLoader, TextureStore};
use crate::engine::asset::typography::TypefaceLoader;
use crate::engine::render::Renderer;
use crate::engine::tile::tileset::TilesetStore;

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
  pub fn new(renderer: &Renderer, ttf_context: &'ttf Sdl2TtfContext) -> Self {
    Self {
      textures: TextureLoader::new(renderer.new_texture_creator()),
      audio: AudioPlayer::new(),
      tilesets: TilesetStore::new(),
      typefaces: TypefaceLoader::new(&ttf_context),
    }
  }

  pub fn load(&mut self, asset_type: AssetType, filepath: String) -> Result<(), &str> {
    match asset_type {
      AssetType::Texture => self.textures.load(filepath).map(|_| ()).map_err(|_| "Failed to load texture"),
      AssetType::Audio { sound_type } => self.audio.load(sound_type, filepath),
      AssetType::Typeface { font_size } => self.typefaces.load(filepath, font_size).map_err(|_e| "Failed to load typeface")
    }
  }

  pub fn use_store(&mut self) -> (&TextureStore, &AudioPlayer, &mut TilesetStore) {
    (&self.textures.use_store(), &self.audio, &mut self.tilesets)
  }
}
