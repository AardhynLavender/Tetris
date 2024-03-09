use crate::engine::event::EventStore;
use crate::engine::manager::assets::AssetManager;
use crate::engine::render::{Properties, Renderer};
use crate::engine::subsystem::Subsystem;

pub struct Actions<TState> {
  /// Set up the games static assets
  pub load: fn(&mut AssetManager),
  /// Render state and assets
  pub render: fn(&mut TState, &AssetManager, &mut Renderer),
  /// Update engine state
  pub update: fn(&EventStore, &AssetManager, &mut TState),
  /// Set up the state
  pub setup: fn(&AssetManager) -> TState,
}

struct Application<'a, TState> {
  subsystem: &'a mut Subsystem,
  actions: Actions<TState>,
  event_store: EventStore,
}

impl<'a, TState> Application<'a, TState> {
  fn new(subsystem: &'a mut Subsystem, actions: Actions<TState>) -> Self {
    Self {
      subsystem,
      actions,
      event_store: EventStore::new(),
    }
  }

  pub fn run(&mut self, assets: &mut AssetManager) -> Result<(), &str> {
    (self.actions.load)(assets);

    let mut state = (self.actions.setup)(assets);

    loop {
      self.subsystem.events.update(&mut self.event_store);
      if self.subsystem.events.is_quit {
        break;
      }

      (self.actions.update)(&self.event_store, assets, &mut state);
      (self.actions.render)(&mut state, assets, &mut self.subsystem.renderer);

      self.subsystem.renderer.present();
    }

    Ok(())
  }
}

pub fn run_application<TState>(properties: Properties, actions: Actions<TState>) {
  let mut subsystem = Subsystem::new(properties);
  let ttf_context = sdl2::ttf::init().expect("failed to initialize ttf context");
  let mut assets = AssetManager::new(&subsystem.renderer, &ttf_context);

  let mut app = Application::new(
    &mut subsystem,
    actions,
  );

  app.run(&mut assets).expect("failed to run application");
}