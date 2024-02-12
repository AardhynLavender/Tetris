use sdl2::image::{InitFlag, LoadTexture};

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

  renderer: Renderer,
  texture_loader: TextureLoader,

  pub loader: fn(&mut TextureLoader),
  pub updater: fn(&EventStore, &TextureStore, &mut Renderer),
}

impl Application {
  pub fn new(properties: ApplicationProperties) -> Self {
    let context = sdl2::init().unwrap();
    sdl2::image::init(InitFlag::PNG).unwrap();

    let events = Events::new(&context);
    let event_store = EventStore::new();

    let renderer = Renderer::new(&context, properties.clone());
    let texture_loader = TextureLoader::new(renderer.new_texture_creator());

    Self { context, renderer, texture_loader, event_store, events, updater: |_, _2, _3| {}, loader: |_| {} }
  }

  pub fn on_load_resources(&mut self, loader: fn(&mut TextureLoader)) -> &mut Self {
    self.loader = loader;
    self
  }

  pub fn on_update(&mut self, updater: fn(events: &EventStore, textures: &TextureStore, renderer: &mut Renderer)) -> &mut Self {
    self.updater = updater;
    self
  }

  pub fn run(&mut self) -> Result<(), ()> {
    (self.loader)(&mut self.texture_loader);
    let store = self.texture_loader.use_store();

    loop {
      self.events.update(&mut self.event_store);
      if self.events.is_quit { break; }

      (self.updater)(&self.event_store, &store, &mut self.renderer);

      self.renderer.present();
    }
    Ok(())
  }
}
