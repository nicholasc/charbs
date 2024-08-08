use std::{
  any::TypeId,
  collections::HashMap,
  hash::{Hash, Hasher},
};

#[derive(Debug, PartialEq, Eq)]
pub struct ResourceId<T> {
  label: &'static str,
  type_id: TypeId,
  _marker: std::marker::PhantomData<T>,
}

impl<T: 'static> ResourceId<T> {
  pub fn new(label: &'static str) -> Self {
    ResourceId {
      label,
      type_id: TypeId::of::<T>(),
      _marker: std::marker::PhantomData,
    }
  }
}

impl<T> Hash for ResourceId<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.label.hash(state);
    self.type_id.hash(state);
  }
}

impl<T: 'static> From<&'static str> for ResourceId<T> {
  fn from(value: &'static str) -> Self {
    Self::new(value)
  }
}

pub struct Resources<T> {
  resources: HashMap<ResourceId<T>, T>,
}

impl<T> Default for Resources<T> {
  fn default() -> Self {
    Self {
      resources: HashMap::new(),
    }
  }
}

impl<T: 'static> Resources<T> {
  pub fn add(&mut self, label: ResourceId<T>, resource: T)
  where
    T: Eq,
  {
    self.resources.insert(label, resource);
  }
}
