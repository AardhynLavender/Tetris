use crate::engine::event::Events;
use crate::engine::render::{Properties, Renderer};

pub struct Subsystem {
  pub sdl_context: sdl2::Sdl,
  pub renderer: Renderer,
  pub events: Events,
}

impl Subsystem {
  pub fn new<'a, 'b>(properties: Properties) -> Self {
    let sdl_context = sdl2::init().expect("failed to initialize SDL2");
    sdl_context.audio().expect("failed to initialize audio");

    let renderer = Renderer::new(&sdl_context, properties);

    let events = Events::new(&sdl_context);

    Self {
      sdl_context,
      renderer,
      events,
    }
  }
}
