use sdl2::image::LoadTexture;

use crate::application::asset::{audio::AudioPlayer, texture::TextureLoader};
use crate::application::event::{Events, EventStore};
use crate::application::manager::{assets::AssetManager, sprite::SpriteManager};
use crate::application::render::{Properties as ApplicationProperties, Renderer};

pub mod asset;
pub mod event;
pub mod geometry;
pub mod render;
pub mod tile;
pub mod utility;
pub mod manager;
mod structure;

// Injector Types
type LoaderFn = fn(&mut AssetManager);
type StateFn<T> = fn() -> T;
type StartFn<TState> = fn(&AssetManager, &mut SpriteManager, &TState);
type UpdateFn = fn(&EventStore, &SpriteManager, &AssetManager, &mut Renderer);
type QuitFn = fn();

pub struct Application<TState: Default> {
  loader: LoaderFn,
  assets: AssetManager,
  sprites: SpriteManager,

  initialize_state: StateFn<TState>,
  state: TState,

  start: StartFn<TState>,

  events: Events,
  event_store: EventStore,

  context: sdl2::Sdl,
  renderer: Renderer,

  updater: UpdateFn,

  quit: QuitFn,
}

impl<TState: Default> Application<TState> {
  pub fn new(properties: ApplicationProperties) -> Self {
    let context = sdl2::init().unwrap();
    context.audio().unwrap();

    let events = Events::new(&context);
    let event_store = EventStore::new();

    let renderer = Renderer::new(&context, properties.clone());

    let state = TState::default();

    let assets = AssetManager::new(
      TextureLoader::new(renderer.new_texture_creator()),
      AudioPlayer::new(),
    );
    let sprites = SpriteManager::new();

    Self {
      context,
      renderer,
      assets,
      sprites,

      event_store,
      events,

      initialize_state: || { TState::default() },
      state,

      loader: |_| {},
      start: |_, _, _| {},
      updater: |_, _, _, _| {},
      quit: || {},
    }
  }

  // Injectors //

  pub fn on_load_assets(&mut self, loader: LoaderFn) -> &mut Self {
    self.loader = loader;
    self
  }
  pub fn use_state(&mut self, initialize_state: StateFn<TState>) -> &mut Self {
    self.initialize_state = initialize_state;
    self
  }
  pub fn on_start(&mut self, start: StartFn<TState>) -> &mut Self {
    self.start = start;
    self
  }
  pub fn on_update(&mut self, updater: UpdateFn) -> &mut Self {
    self.updater = updater;
    self
  }
  pub fn on_quit(&mut self, quit: QuitFn) -> &mut Self {
    self.quit = quit;
    self
  }

  // Execution //

  pub fn run(&mut self) -> Result<(), ()> {
    // load user resources
    (self.loader)(&mut self.assets);

    // load user initial state
    self.state = (self.initialize_state)();

    let mut sprites = SpriteManager::new();

    // start application
    (self.start)(&self.assets, &mut sprites, &self.state);

    loop {
      self.events.update(&mut self.event_store);
      if self.events.is_quit { break; }

      // user defined update
      (self.updater)(
        &self.event_store,
        &mut sprites,
        &self.assets,
        &mut self.renderer,
      );

      // todo: ensure consistent frame rate with accumulator and fixed time step

      self.renderer.present();
    }
    Ok(())
  }
}
