use std::collections::HashMap;

/// A structure to represent an asset.
///
/// At the core, an asset is a byte array but the structure also provides
/// methods to convert it to different formats. Asset can also be extended from
/// the outside to provide more possible conversions.
#[derive(Default)]
pub struct Asset {
  data: Vec<u8>,
}

/// Implement the `AsRef<str>` trait for the [`Asset`] struct.
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
