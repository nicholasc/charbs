use std::collections::HashMap;

#[derive(Default)]
pub struct Asset {
  pub data: Vec<u8>,
}

impl AsRef<str> for Asset {
  fn as_ref(&self) -> &str {
    std::str::from_utf8(&self.data).unwrap()
  }
}

#[derive(Default)]
pub struct Assets {
  storage: HashMap<&'static str, Asset>,
}

impl Assets {
  #[inline]
  pub fn load(&mut self, path: &'static str) {
    if !self.storage.contains_key(path) {
      self.storage.insert(
        path,
        Asset {
          data: std::fs::read(path).unwrap(),
        },
      );
    }
  }

  pub fn get(&mut self, path: &'static str) -> &Asset {
    self.load(path);
    self.storage.get(path).unwrap()
  }

  pub fn unload(&mut self, path: &'static str) {
    self.storage.remove(&path);
  }

  pub fn clear(&mut self) {
    self.storage.clear();
  }
}
