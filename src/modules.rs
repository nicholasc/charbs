use crate::app::App;

pub trait Module {
  fn build(&self, app: &mut App);
}
