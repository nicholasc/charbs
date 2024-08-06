#[derive(Clone, Debug, PartialEq)]
pub struct AssetId<T> {
  id: usize,
  _marker: std::marker::PhantomData<T>,
}

impl<T> std::ops::Deref for AssetId<T> {
  type Target = usize;

  fn deref(&self) -> &Self::Target {
    &self.id
  }
}

pub struct Assets<T> {
  data: Vec<Option<T>>,
  reuse: Vec<AssetId<T>>,
}

impl<T> Assets<T> {
  pub fn new() -> Self {
    Self {
      data: Vec::new(),
      reuse: Vec::new(),
    }
  }

  pub fn add(&mut self, asset: T) -> AssetId<T> {
    let id = self.reuse.pop().unwrap_or_else(|| {
      let id = AssetId {
        id: self.data.len(),
        _marker: std::marker::PhantomData,
      };

      self.data.push(None);

      id
    });

    self.data[*id] = Some(asset);

    id
  }

  pub fn remove(&mut self, id: &AssetId<T>) {
    if let Some(data) = self.data.get_mut(**id) {
      if let Some(_) = data {
        *data = None;
        self.reuse.push(AssetId {
          id: **id,
          _marker: std::marker::PhantomData,
        });
      }
    }
  }

  pub fn get(&self, id: &AssetId<T>) -> Option<&T> {
    self.data.get(**id).unwrap().as_ref()
  }
}
