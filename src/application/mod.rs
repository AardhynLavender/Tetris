use crate::application::events::{EventStore, Events};
use crate::application::geometry::{Line2, Pol2, Rec2, Vec2};
use crate::application::render::color::color;
use crate::application::render::{Renderer, RendererProperties as ApplicationProperties};

mod asset;
pub mod events;
pub mod geometry;
pub mod render;

const FRAME_WAIT: u32 = 1_000_000_000u32 / 30;

pub struct Application {
    subsystem: sdl2::Sdl,
    pub renderer: Renderer,
    pub event_store: EventStore,
    pub events: Events,
}

impl Application {
    /// Construct a new `Application` instance
    pub fn new(properties: ApplicationProperties) -> Self {
        let subsystem = sdl2::init().unwrap();

        let events = Events::new(&subsystem);
        let event_store = EventStore::new();

        let renderer = Renderer::new(&subsystem, properties.clone());

        subsystem.mouse().show_cursor(properties.show_cursor);

        Self {
            subsystem,
            renderer,
            event_store,
            events,
        }
    }

    fn render(&mut self) {
        let mouse = self.event_store.get_mouse_position();

        let rect = Rec2::new(mouse, Vec2::new(100u8, 200u8));
        self.renderer.draw_rect(rect, color::GREEN);

        let line = Line2::new(Vec2::default(), mouse);
        self.renderer.draw_line(line, color::RED);

        let polygon = Pol2::new(vec![
            Vec2::new(100, 100),
            Vec2::new(200, 100),
            Vec2::new(100, 200),
        ]);
        self.renderer.draw_poly(polygon, color::CYAN);

        self.renderer.present();
    }

    pub fn run(&mut self) -> Result<(), ()> {
        loop {
            self.events.update(&mut self.event_store);
            if self.events.is_quit {
                return Ok(());
            }

            // todo: implement fixed timestep rendering

            // update...

            self.render();
        }
    }
}
