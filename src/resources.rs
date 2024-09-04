pub struct ResourceHandle<T> {
  index: usize,
  _marker: std::marker::PhantomData<T>,
}

#[derive(Default)]
pub struct Resources<T> {
  storage: Vec<T>,
}

impl<T> Resources<T> {
  pub fn add(&mut self, resource: impl Into<T>) -> ResourceHandle<T> {
    let index = self.storage.len();

    self.storage.push(resource.into());

    ResourceHandle {
      index,
      _marker: std::marker::PhantomData,
    }
  }

  pub fn get(&self, handle: &ResourceHandle<T>) -> Option<&T> {
    self.storage.get(handle.index)
  }
}
