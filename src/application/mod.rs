use sdl2::image::LoadTexture;

use crate::application::asset::{audio::AudioPlayer, texture::TextureLoader};
use crate::application::event::{Events, EventStore};
use crate::application::manager::{assets::AssetManager, object::ObjectManager};
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
type StartFn<TState> = fn(&AssetManager, &mut ObjectManager<TState>, &TState);
type UpdateFn<TState> = fn(&EventStore, &ObjectManager<TState>, &AssetManager, &mut Renderer);
type QuitFn = fn();

pub struct Application<TState: Default> {
  context: sdl2::Sdl,
  renderer: Renderer,

  loader: Option<LoaderFn>,
  start: Option<StartFn<TState>>,
  initialize_state: Option<StateFn<TState>>,
  updater: Option<UpdateFn<TState>>,
  quit: Option<QuitFn>,

  assets: AssetManager,
  objects: ObjectManager<TState>,
  events: Events,
  event_store: EventStore,

  state: TState,
}

impl<TState: Default> Application<TState> {
  pub fn new(properties: ApplicationProperties) -> Self {
    let context = sdl2::init().unwrap();
    context.audio().unwrap();

    let events = Events::new(&context);
    let event_store = EventStore::new();
    let renderer = Renderer::new(&context, properties.clone());
    let texture_loader = TextureLoader::new(renderer.new_texture_creator());
    let audio_player = AudioPlayer::new();
    let assets = AssetManager::new(texture_loader, audio_player);
    let state = TState::default();
    let objects = ObjectManager::new();

    Self {
      context,
      renderer,
      assets,
      objects,

      event_store,
      events,

      initialize_state: None,
      state,

      loader: None,
      start: None,
      updater: None,
      quit: None,
    }
  }

  // Injectors //

  pub fn on_load_assets(&mut self, loader: LoaderFn) -> &mut Self {
    self.loader = Some(loader);
    self
  }
  pub fn use_state(&mut self, initialize_state: StateFn<TState>) -> &mut Self {
    self.initialize_state = Some(initialize_state);
    self
  }
  pub fn on_start(&mut self, start: StartFn<TState>) -> &mut Self {
    self.start = Some(start);
    self
  }
  pub fn on_update(&mut self, updater: UpdateFn<TState>) -> &mut Self {
    self.updater = Some(updater);
    self
  }
  pub fn on_quit(&mut self, quit: QuitFn) -> &mut Self {
    self.quit = Some(quit);
    self
  }

  // Execution //

  pub fn run(&mut self) -> Result<(), ()> {
    // load user resources
    if let Some(loader) = self.loader {
      (loader)(&mut self.assets);
    }

    // load user initial state
    self.state = if let Some(initialize_state) = self.initialize_state {
      (initialize_state)()
    } else {
      // todo: this is redundant
      TState::default()
    };

    // start application
    if let Some(start) = self.start {
      (start)(&self.assets, &mut self.objects, &self.state);
    }

    // unwrap here to avoid repeated checks
    let update = if let Some(updater) = self.updater {
      updater
    } else {
      return Err(());
    };

    loop {
      self.events.update(&mut self.event_store);
      if self.events.is_quit { break; }

      self.objects.event(&self.event_store);
      self.objects.update(&mut self.state);

      // user defined update
      (update)(
        &self.event_store,
        &mut self.objects,
        &self.assets,
        &mut self.renderer,
      );

      // todo: ensure consistent frame rate with accumulator and fixed time step

      self.objects.render(&mut self.renderer);
      self.renderer.present();
    }
    Ok(())
  }
}
