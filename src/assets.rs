#[derive(Clone, Debug, PartialEq)]
pub struct AssetId {
  id: usize,
}

impl std::ops::Deref for AssetId {
  type Target = usize;

  fn deref(&self) -> &Self::Target {
    &self.id
  }
}

pub struct Assets<T> {
  data: Vec<Option<T>>,
  reuse: Vec<AssetId>,
}

impl<T> Assets<T> {
  pub fn new() -> Self {
    Self {
      data: Vec::new(),
      reuse: Vec::new(),
    }
  }

  pub fn add(&mut self, asset: T) -> AssetId {
    let id = self.reuse.pop().unwrap_or_else(|| {
      let id = AssetId {
        id: self.data.len(),
      };

      self.data.push(None);

      id
    });

    self.data[*id] = Some(asset);

    id
  }

  pub fn remove(&mut self, id: &AssetId) {
    if let Some(data) = self.data.get_mut(**id) {
      if let Some(_) = data {
        *data = None;
        self.reuse.push(id.clone());
      }
    }
  }

  pub fn get(&self, id: &AssetId) -> Option<&T> {
    self.data.get(**id).unwrap().as_ref()
  }
}
