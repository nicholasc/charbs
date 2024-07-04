use charbs::state::{Res, ScheduleLabel, Scheduler, State};

#[derive(ScheduleLabel)]
pub struct Init;

#[derive(ScheduleLabel)]
pub struct Update;

fn init(message: Res<&str>) {
  println!("Init: {}", *message);
}

fn update(message: Res<&str>) {
  println!("Update: {}", *message);
}

fn main() {
  let mut i = 0;

  let mut state = State::default();
  state.add("Hello, world!");

  let mut scheduler = Scheduler::default();
  scheduler.add_handler(Init, init);
  scheduler.add_handler(Update, update);

  scheduler.run(Init, &mut state);

  loop {
    scheduler.run(Update, &mut state);
    i += 1;

    if i == 10 {
      break;
    }
  }
}
