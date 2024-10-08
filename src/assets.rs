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

/// A structure used to manage on disk assets.
#[derive(Default)]
pub struct Assets {
  storage: HashMap<&'static str, Asset>,
}

impl Assets {
  /// Load an asset from a file path. If the asset already exists in memory, it
  /// will not be loaded.
  ///
  /// TODO: Implement error handling for file reading failures.
  ///
  /// # Arguments
  ///
  /// * `path` - The file path of the asset to load.
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

  /// Return a reference to the asset with the given path. If the asset does not
  /// exist, it will be loaded.
  ///
  /// # Arguments
  ///
  /// * `path` - The path of the asset to retrieve.
  ///
  /// * `->` - A reference to the asset with the given path.
  pub fn get(&mut self, path: &'static str) -> &Asset {
    self.load(path);
    self.storage.get(path).unwrap()
  }

  /// Unload the asset with the given path from memory.
  ///
  /// # Arguments
  ///
  /// * `path` - The path of the asset to unload.
  pub fn unload(&mut self, path: &'static str) {
    self.storage.remove(&path);
  }

  /// Unload all assets from memory.
  pub fn clear(&mut self) {
    self.storage.clear();
  }
}
