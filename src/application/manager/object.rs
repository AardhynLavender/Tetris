use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::application::event::EventStore;
use crate::application::render::Renderer;

pub trait Object<TState> {
  fn update(&mut self, state: &mut TState);
  fn render(&self, renderer: &mut Renderer);
  fn event(&mut self, event_store: &EventStore);
}

// Easier to reimplement a store than try bootstrap reference cells and dynamic dispatch onto Store<T>
type ObjectStore<T> = HashMap<String, Rc<RefCell<dyn Object<T>>>>;

pub struct ObjectManager<TState> {
  objects: ObjectStore<TState>,
}

impl<TState> ObjectManager<TState> {
  pub fn new() -> Self {
    Self { objects: HashMap::new() }
  }

  // Mutators //

  pub fn add(&mut self, name: String, object: Rc<RefCell<dyn Object<TState>>>) -> &mut Rc<RefCell<dyn Object<TState>>> {
    self.objects.entry(name).or_insert(object)
  }
  pub fn get(&self, name: &str) -> Option<Rc<RefCell<dyn Object<TState>>>> {
    self.objects.get(name).map(|object| Rc::clone(object))
  }

  // Behavior //

  pub fn update(&mut self, state: &mut TState) {
    for object in &mut self.objects.values_mut() {
      object.borrow_mut().update(state);
    }
  }
  pub fn render(&self, renderer: &mut Renderer) {
    for object in self.objects.values() {
      object.borrow().render(renderer);
    }
  }
  pub fn event(&mut self, event_store: &EventStore) {
    for object in &mut self.objects.values_mut() {
      object.borrow_mut().event(event_store);
    }
  }
}