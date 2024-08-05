#[cfg(test)]
mod tests {
  use charbs::assets::*;

  #[derive(Clone, Debug, PartialEq)]
  pub struct MyAsset {
    data: u32,
  }

  #[test]
  fn assets_works() {
    let mut assets = Assets::<MyAsset>::new();
    let asset = MyAsset { data: 42 };

    let id = assets.add(asset.clone());

    assert_eq!(assets.get(&id), Some(&asset));

    assets.remove(&id);

    assert_eq!(assets.get(&id), None);
  }

  #[test]
  fn assets_reuses_ids() {
    let mut assets = Assets::<MyAsset>::new();

    let asset1 = MyAsset { data: 1 };
    let asset2 = MyAsset { data: 2 };
    let asset3 = MyAsset { data: 3 };

    let id1 = assets.add(asset1.clone());
    let id2 = assets.add(asset2.clone());

    assert_ne!(id1, id2);

    assets.remove(&id1);

    let id3 = assets.add(asset3.clone());

    dbg!(&id1, &id3);

    assert_eq!(id1, id3);
    assert_eq!(assets.get(&id1), Some(&asset3));
    assert_eq!(assets.get(&id2), Some(&asset2));
    assert_eq!(assets.get(&id3), Some(&asset3));

    assets.remove(&id2);

    assert_eq!(assets.get(&id2), None);

    let id4 = assets.add(MyAsset { data: 4 });

    assert_eq!(id2, id4);
  }
}
