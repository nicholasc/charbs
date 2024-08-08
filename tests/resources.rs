#[cfg(test)]

mod tests {
  use std::hash::{DefaultHasher, Hash, Hasher};

  use charbs::resources::*;

  #[derive(Debug, PartialEq, Eq)]
  struct TestResource;

  #[derive(Debug, PartialEq, Eq)]
  struct TestResource2;

  #[test]
  fn resource_id_eq() {
    let id1 = ResourceId::<TestResource>::new("texture");
    let id2 = ResourceId::<TestResource>::new("texture");

    assert_eq!(id1, id2);
  }

  #[test]
  fn resource_id_hash_eq() {
    let id1 = ResourceId::<TestResource>::new("texture");
    let id2 = ResourceId::<TestResource>::new("texture");

    let mut state1 = DefaultHasher::new();
    let mut state2 = DefaultHasher::new();

    id1.hash(&mut state1);
    id2.hash(&mut state2);

    let hash1 = state1.finish();
    let hash2 = state2.finish();

    assert_eq!(hash1, hash2);
  }

  #[test]
  fn resource_id_hash_not_eq_label() {
    let id1 = ResourceId::<TestResource>::new("texture");
    let id2 = ResourceId::<TestResource>::new("material");

    let mut state1 = DefaultHasher::new();
    let mut state2 = DefaultHasher::new();

    id1.hash(&mut state1);
    id2.hash(&mut state2);

    let hash1 = state1.finish();
    let hash2 = state2.finish();

    assert_ne!(hash1, hash2);
  }

  #[test]
  fn resource_id_hash_not_eq_type() {
    let id1 = ResourceId::<TestResource>::new("texture");
    let id2 = ResourceId::<TestResource2>::new("texture");

    let mut state1 = DefaultHasher::new();
    let mut state2 = DefaultHasher::new();

    id1.hash(&mut state1);
    id2.hash(&mut state2);

    let hash1 = state1.finish();
    let hash2 = state2.finish();

    assert_ne!(hash1, hash2);
  }
}
