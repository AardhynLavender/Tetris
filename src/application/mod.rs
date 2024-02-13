use sdl2::image::LoadTexture;

use crate::application::asset::audio::{AudioLoader, AudioStore};
use crate::application::asset::texture::{TextureLoader, TextureStore};
use crate::application::events::{Events, EventStore};
use crate::application::render::{Properties as ApplicationProperties, Renderer};

pub mod asset;
pub mod events;
pub mod geometry;
pub mod render;

pub struct Application {
  context: sdl2::Sdl,

  events: Events,
  event_store: EventStore,

  audio_loader: AudioLoader,

  renderer: Renderer,
  texture_loader: TextureLoader,

  pub loader: fn(&mut TextureLoader, &mut AudioLoader),
  pub updater: fn(&EventStore, &TextureStore, &AudioStore, &mut Renderer),
}

impl Application {
  pub fn new(properties: ApplicationProperties) -> Self {
    let context = sdl2::init().unwrap();
    context.audio().unwrap();

    let events = Events::new(&context);
    let event_store = EventStore::new();

    let renderer = Renderer::new(&context, properties.clone());
    let texture_loader = TextureLoader::new(renderer.new_texture_creator());

    let audio_loader = AudioLoader::new();

    Self { context, renderer, texture_loader, audio_loader, event_store, events, updater: |_, _2, _3, _4| {}, loader: |_, _2| {} }
  }

  pub fn on_load_resources(&mut self, loader: fn(&mut TextureLoader, &mut AudioLoader)) -> &mut Self {
    self.loader = loader;
    self
  }

  pub fn on_update(&mut self, updater: fn(events: &EventStore, textures: &TextureStore, audio: &AudioStore, renderer: &mut Renderer)) -> &mut Self {
    self.updater = updater;
    self
  }

  pub fn run(&mut self) -> Result<(), ()> {
    (self.loader)(&mut self.texture_loader, &mut self.audio_loader);
    let texture_store = self.texture_loader.use_store();
    let audio_store = self.audio_loader.use_store();

    loop {
      self.events.update(&mut self.event_store);
      if self.events.is_quit { break; }

      (self.updater)(&self.event_store, &texture_store, &audio_store, &mut self.renderer);

      self.renderer.present();
    }
    Ok(())
  }
}
