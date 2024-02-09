use std::collections::hash_set::HashSet;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::application::geometry::Vec2;

pub struct EventStore {
  pressed_keys: HashSet<Keycode>,
  mouse_position: Vec2<i32>,
}

impl EventStore {
  /// Construct an `EventStore` instance
  pub fn new() -> Self {
    Self {
      pressed_keys: HashSet::new(),
      mouse_position: Vec2::default(),
    }
  }
  /// press `keycode`
  pub fn press_key(&mut self, keycode: Keycode) {
    self.pressed_keys.insert(keycode);
  }
  /// Raise `keycode`
  pub fn raise_key(&mut self, keycode: Keycode) {
    self.pressed_keys.remove(&keycode);
  }
  /// Check if `keycode` is pressed
  pub fn is_pressed(&self, keycode: Keycode) -> bool {
    self.pressed_keys.contains(&keycode)
  }
  /// Check where the mouse is
  pub fn get_mouse_position(&self) -> Vec2<i32> {
    self.mouse_position
  }
  /// Set the current mouse position
  pub fn set_mose_position(&mut self, position: Vec2<i32>) {
    self.mouse_position = position;
  }
}

pub struct Events {
  event_pump: sdl2::EventPump,
  pub is_quit: bool,
}

impl Events {
  /// construct an `Events` instance from `event_properties`
  pub fn new(context: &sdl2::Sdl) -> Self {
    let event_pump = context.event_pump().unwrap();
    Self {
      event_pump,
      is_quit: false,
    }
  }

  /// Update a `keystore` with polled events
  pub fn update(&mut self, event_store: &mut EventStore) {
    let events = self.event_pump.poll_iter();
    for event in events {
      match event {
        Event::Quit { .. } => {
          self.is_quit = true;
        }
        Event::KeyDown { keycode, .. } => {
          keycode.map(|keycode| event_store.press_key(keycode));
        }
        Event::KeyUp { keycode, .. } => {
          keycode.map(|keycode| event_store.raise_key(keycode));
        }
        Event::MouseMotion { x, y, .. } => {
          event_store.set_mose_position(Vec2 { x, y });
        }
        _ => {}
      }
    }
  }
}
