use std::collections::HashMap;
use std::rc::Rc;

pub struct HeapStore<T: ?Sized> {
  pub store: HashMap<String, Rc<T>>,
}

impl<T: ?Sized> HeapStore<T> {
  pub fn new() -> Self {
    Self { store: HashMap::new() }
  }

  // Mutators //

  pub fn add(&mut self, key: String, value: Rc<T>) -> Rc<T> {
    Rc::clone(self.store.entry(key).or_insert(value))
  }

  // Accessors //

  pub fn get(&self, key: &str) -> Result<Rc<T>, String> {
    return if let Some(value) = self.store.get(key) {
      Ok(Rc::clone(value))
    } else {
      Err(format!("Failed to get {} from store", key))
    };
  }
}

// Stack Store //

pub struct Store<T> {
  pub store: HashMap<String, T>,
}

impl<T> Store<T> {
  pub fn new() -> Self {
    Self { store: HashMap::new() }
  }

  // Mutators //

  pub fn add(&mut self, key: String, value: T) -> &mut T {
    self.store.entry(key).or_insert(value)
  }

  // Accessors //

  pub fn get(&self, key: &str) -> Result<&T, String> {
    return if let Some(value) = self.store.get(key) {
      Ok(value)
    } else {
      Err(format!("Failed to get {} from store", key))
    };
  }
}

