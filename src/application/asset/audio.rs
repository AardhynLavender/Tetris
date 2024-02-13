use std::collections::HashMap;

use sdl2::mixer::DEFAULT_CHANNELS;

/// 44.1 kHz
pub const FREQUENCY: i32 = 44_100;
/// Signed 16-bit samples
pub const FORMAT: sdl2::mixer::AudioFormat = sdl2::mixer::DEFAULT_FORMAT;
/// 2 channels (stereo)
pub const OUTPUT_CHANNELS: i32 = DEFAULT_CHANNELS;
/// Number of channels available for mixing sound effects
pub const MIXER_CHANNELS: i32 = 16;
/// Samples processed per frame
pub const CHUNK_SIZE: i32 = 2048;

pub enum SoundType {
  Music,
  Effect,
}

pub enum Sound {
  Music { data: sdl2::mixer::Music<'static> },
  Effect { data: sdl2::mixer::Chunk },
}

pub struct Audio {
  pub sound: Sound,
  pub name: String,
  pub path: String,
}

pub struct AudioStore {
  pub sounds: HashMap<String, Audio>,
}

pub enum Loop {
  Forever,
  Once,
  For { times: i32 },
}

impl AudioStore {
  pub fn new() -> Self { Self { sounds: HashMap::new() } }
  pub fn add(&mut self, name: String, audio: Audio) -> &Audio { self.sounds.entry(name).or_insert(audio) }
  pub fn play(&self, name: &str, volume: i32, looping: Loop) -> Result<(), &str> {
    let mut audio = self.sounds.get(name).ok_or("Failed to get audio")?;
    let loops = match looping {
      Loop::Forever => -1,
      Loop::Once => 0,
      Loop::For { times } => times,
    };

    match &audio.sound {
      Sound::Music { data } => {
        sdl2::mixer::Music::set_volume(volume);
        data.play(loops).map_err(|_| "Failed to play music")?;
        Ok(())
      }
      Sound::Effect { data } => {
        if let channel = sdl2::mixer::Channel::all().play(data, loops).expect("Failed to play effect") {
          channel.set_volume(volume);
        }
        Ok(())
      }
    }
  }
}

pub struct AudioLoader {
  store: AudioStore,
}

impl AudioLoader {
  pub fn new() -> Self {
    initialize_audio_subsystem().expect("Failed to initialize audio subsystem");
    Self { store: AudioStore::new() }
  }

  pub fn load(&mut self, sound_type: SoundType, filepath: String) -> Result<(), &str> {
    let path = filepath.clone();
    let filename = path.split("/").last().ok_or("Failed to get filename")?;
    let basename = filename.split(".").next().ok_or("Failed to get basename")?;

    match sound_type {
      SoundType::Music => {
        let music = sdl2::mixer::Music::from_file(filepath.clone()).expect("Failed to load music");
        let audio = Audio { sound: Sound::Music { data: music }, name: String::from(basename), path: filepath };
        self.store.add(String::from(basename), audio);
        Ok(())
      }
      SoundType::Effect => {
        let effect = sdl2::mixer::Chunk::from_file(filepath.clone()).expect("Failed to load sound effect");
        let audio = Audio { sound: Sound::Effect { data: effect }, name: String::from(basename), path: filepath };
        self.store.add(String::from(basename), audio);
        Ok(())
      }
    }
  }

  pub fn use_store(&self) -> &AudioStore { &self.store }
}

/// Initialize the SDL_Mixer audio subsystem
fn initialize_audio_subsystem() -> Result<(), String> {
  sdl2::mixer::open_audio(FREQUENCY, sdl2::mixer::DEFAULT_FORMAT, OUTPUT_CHANNELS, CHUNK_SIZE)?;
  sdl2::mixer::init(sdl2::mixer::InitFlag::all())?;
  sdl2::mixer::allocate_channels(MIXER_CHANNELS);
  Ok(())
}