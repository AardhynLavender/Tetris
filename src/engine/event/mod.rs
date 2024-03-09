use std::collections::hash_set::HashSet;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::engine::geometry::Vec2;

type KeyStore = HashSet<Keycode>;

pub struct EventStore {
  held_keys: KeyStore,
  pressed_keys: KeyStore,
  mouse_position: Vec2<i32>,
}

impl EventStore {
  pub fn new() -> Self {
    Self {
      pressed_keys: HashSet::new(),
      held_keys: HashSet::new(),
      mouse_position: Vec2::default(),
    }
  }

  // Mutators //

  pub fn clear_pressed_keys(&mut self) {
    self.pressed_keys.clear();
  }
  pub fn press_key(&mut self, keycode: Keycode) {
    self.pressed_keys.insert(keycode);
    self.held_keys.insert(keycode);
  }
  pub fn raise_key(&mut self, keycode: Keycode) {
    // no need to remove from `pressed_keys` as it will be cleared at the start of the next frame
    self.held_keys.remove(&keycode);
  }
  pub fn set_mose_position(&mut self, position: Vec2<i32>) {
    self.mouse_position = position;
  }

  // Accessors //

  /// Returns true if the key was pressed this frame.
  pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
    self.pressed_keys.contains(&keycode)
  }
  /// Returns true if the key is currently held down.
  pub fn is_key_held(&self, keycode: Keycode) -> bool {
    self.held_keys.contains(&keycode)
  }
  pub fn get_mouse_position(&self) -> Vec2<i32> {
    self.mouse_position
  }
}

pub struct Events {
  event_pump: sdl2::EventPump,
  pub is_quit: bool,
}

impl Events {
  pub fn new(context: &sdl2::Sdl) -> Self {
    let event_pump = context.event_pump().unwrap();
    Self {
      event_pump,
      is_quit: false,
    }
  }

  pub fn update(&mut self, event_store: &mut EventStore) {
    event_store.clear_pressed_keys();
    let events = self.event_pump.poll_iter();

    for event in events {
      match event {
        Event::Quit { .. } => {
          self.is_quit = true;
        }
        Event::KeyDown { keycode, .. } => {
          keycode.map(|keycode| {
            if !event_store.is_key_held(keycode) {
              event_store.press_key(keycode);
            }
          });
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
