use charbs::{
  app::{App, Init, Update},
  state::ResMut,
};

use std::error::Error;

fn init() {
  println!("Hello from init!");
}

fn update(mut i: ResMut<i32>) {
  if *i < 10 {
    println!("Hello from update! {}", *i);
    *i += 1;
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::default();

  let scheduler = app.scheduler_mut();
  scheduler.add_handler(Init, init);
  scheduler.add_handler(Update, update);

  let state = app.state_mut();
  state.add(0);

  app.run()
}
