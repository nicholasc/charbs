use charbs::state::{Res, Scheduler, State};

fn init(message: Res<&str>) {
  println!("{}", *message);
}

fn main() {
  let mut state = State::default();
  state.add("Hello, world!");

  let mut scheduler = Scheduler::default();
  scheduler.add_init_handler(init);
  scheduler.run_init(&mut state);
}
