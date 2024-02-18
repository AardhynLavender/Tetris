use sdl2::image::LoadTexture;

use crate::application::asset::audio::AudioLoader;
use crate::application::asset::texture::TextureLoader;
use crate::application::events::{Events, EventStore};
use crate::application::managers::AssetManager;
use crate::application::render::{Properties as ApplicationProperties, Renderer};
use crate::application::tiles::tileset::TilesetStore;

pub mod asset;
pub mod events;
pub mod geometry;
pub mod render;
pub mod tiles;
pub mod utility;
pub mod managers;

// Injector Types
type LoaderFn = fn(&mut AssetManager);
type StateFn<T> = fn() -> T;
type StartFn = fn();
type UpdateFn = fn(&EventStore, &AssetManager, &mut Renderer);
type QuitFn = fn();

pub struct Application<TState: Default> {
  loader: LoaderFn,
  asset_manager: AssetManager,

  initialize_state: StateFn<TState>,
  state: TState,

  start: StartFn,

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
    let texture_loader = TextureLoader::new(renderer.new_texture_creator());
    let audio_loader = AudioLoader::new();
    let tileset_store = TilesetStore::new();
    let asset_loader = AssetManager::new(texture_loader, audio_loader);

    let state = TState::default();

    Self {
      context,
      renderer,
      asset_manager: asset_loader,

      event_store,
      events,

      initialize_state: || { TState::default() },
      state,

      start: || {},
      updater: |_, _, _| {},
      loader: |_| {},
      quit: || {},
    }
  }

  // Injectors //

  pub fn on_load_resources(&mut self, loader: LoaderFn) -> &mut Self {
    self.loader = loader;
    self
  }
  pub fn use_state(&mut self, initialize_state: StateFn<TState>) -> &mut Self {
    self.initialize_state = initialize_state;
    self
  }
  pub fn on_start(&mut self, start: StartFn) -> &mut Self {
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
    (self.loader)(&mut self.asset_manager);

    // load user initial state
    self.state = (self.initialize_state)();

    // start application
    (self.start)();

    loop {
      self.events.update(&mut self.event_store);
      if self.events.is_quit { break; }

      // user defined update
      (self.updater)(
        &self.event_store,
        &self.asset_manager,
        &mut self.renderer,
      );

      // todo: ensure consistent frame rate with accumulator and fixed time step

      self.renderer.present();
    }
    Ok(())
  }
}
