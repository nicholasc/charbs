use charbs::prelude::*;

struct MyModule;

impl Module for MyModule {
  fn build(&self, app: &mut App) {
    app
      .add_resource(0)
      .add_handler(Init, Self::init)
      .add_handler(Update, Self::update);
  }
}

impl MyModule {
  fn init() {
    println!("Hello from MyModule!");
  }

  fn update(mut i: ResMut<i32>) {
    if *i < 10 {
      println!("MyModule update! {}", *i);
      *i += 1;
    }
  }
}

fn main() {
  App::default().add_module(MyModule).run();
}
