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
type StartFn<TState> = fn(&AssetManager, &mut ObjectManager<TState>) -> TState;
type UpdateFn<TState> = fn(&EventStore, &mut ObjectManager<TState>, &mut TState, &mut Renderer);
type RenderFn<TState> = fn(&ObjectManager<TState>, &TState, &mut Renderer);
type QuitFn = fn();

pub struct Application<TState> {
  context: sdl2::Sdl,
  renderer: Renderer,

  load: Option<LoaderFn>,
  start: Option<StartFn<TState>>,
  update: Option<UpdateFn<TState>>,
  render: Option<RenderFn<TState>>,
  quit: Option<QuitFn>,

  assets: AssetManager,
  objects: ObjectManager<TState>,
  events: Events,
  event_store: EventStore,
}

impl<TState> Application<TState> {
  pub fn new(properties: ApplicationProperties) -> Self {
    let context = sdl2::init().unwrap();
    context.audio().unwrap();

    let events = Events::new(&context);
    let event_store = EventStore::new();
    let renderer = Renderer::new(&context, properties.clone());
    let texture_loader = TextureLoader::new(renderer.new_texture_creator());
    let audio_player = AudioPlayer::new();
    let assets = AssetManager::new(texture_loader, audio_player);
    let objects = ObjectManager::new();

    Self {
      context,
      renderer,
      assets,
      objects,

      event_store,
      events,

      render: None,
      load: None,
      start: None,
      update: None,
      quit: None,
    }
  }

  // Injectors //

  pub fn on_load_assets(&mut self, loader: LoaderFn) -> &mut Self {
    self.load = Some(loader);
    self
  }
  pub fn on_start(&mut self, start: StartFn<TState>) -> &mut Self {
    self.start = Some(start);
    self
  }
  pub fn on_update(&mut self, updater: UpdateFn<TState>) -> &mut Self {
    self.update = Some(updater);
    self
  }

  pub fn on_render(&mut self, render: RenderFn<TState>) -> &mut Self {
    self.render = Some(render);
    self
  }
  pub fn on_quit(&mut self, quit: QuitFn) -> &mut Self {
    self.quit = Some(quit);
    self
  }

  // Execution //

  pub fn run(&mut self) -> Result<(), ()> {
    // load user resources
    if let Some(loader) = self.load {
      (loader)(&mut self.assets);
    }

    let start = if let Some(start) = self.start {
      start
    } else {
      return Err(());
    };

    // start application
    let mut state = (start)(&self.assets, &mut self.objects);

    let update = if let Some(updater) = self.update {
      updater
    } else {
      return Err(());
    };

    let render = if let Some(renderer) = self.render {
      renderer
    } else {
      return Err(());
    };

    loop {
      // events
      self.events.update(&mut self.event_store);
      if self.events.is_quit {
        break;
      }
      self.objects.event(&self.event_store);

      // todo: ensure consistent frame rate with accumulator and fixed time step
      // update
      (update)(&self.event_store, &mut self.objects, &mut state, &mut self.renderer);
      self.objects.update(&mut state);

      // render
      (render)(&self.objects, &state, &mut self.renderer);
      self.objects.render(&mut self.renderer);
      self.renderer.present();
    }

    Ok(())
  }
}
