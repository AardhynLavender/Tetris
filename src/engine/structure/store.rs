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
  pub fn remove(&mut self, key: &str) -> Result<Rc<T>, &'static str> {
    self.store.remove(key).ok_or("Failed to remove from store")
  }
  pub fn clear(&mut self) {
    self.store.clear()
  }

  // Accessors //

  pub fn get(&self, key: &str) -> Result<Rc<T>, String> {
    return if let Some(value) = self.store.get(key) {
      Ok(Rc::clone(value))
    } else {
      Err(format!("Failed to get {} from store", key))
    };
  }
  pub fn get_mut(&mut self, key: &str) -> Result<&mut Rc<T>, String> {
    match self.store.get_mut(key) {
      Some(value) => Ok(value),
      None => Err(format!("Failed to get `{}` from store", key)),
    }
  }

  // Queries //

  pub fn contains(&self, key: &str) -> bool {
    self.store.contains_key(key)
  }
  pub fn len(&self) -> usize {
    self.store.len()
  }
  pub fn is_empty(&self) -> bool {
    self.store.is_empty()
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
  pub fn remove(&mut self, key: &str) -> Result<T, &'static str> {
    self.store.remove(key).ok_or("Failed to remove from store")
  }
  pub fn clear(&mut self) {
    self.store.clear()
  }

  // Accessors //

  pub fn get(&self, key: &str) -> Result<&T, String> {
    return if let Some(value) = self.store.get(key) {
      Ok(value)
    } else {
      Err(format!("Failed to get {} from store", key))
    };
  }
  pub fn get_mut(&mut self, key: &str) -> Result<&mut T, String> {
    match self.store.get_mut(key) {
      Some(value) => Ok(value),
      None => Err(format!("Failed to get `{}` from store", key)),
    }
  }

  // Queries //

  pub fn contains(&self, key: &str) -> bool {
    self.store.contains_key(key)
  }
  pub fn len(&self) -> usize {
    self.store.len()
  }
  pub fn is_empty(&self) -> bool {
    self.store.is_empty()
  }
}

