use std::sync::{Arc, Mutex};

use crate::{
  modules::Module,
  state::{Handler, IntoHandler, ScheduleLabel, Scheduler, State},
};

/// A schedule label that represents the application initialization schedule.
#[derive(ScheduleLabel)]
pub struct Init;

/// A schedule label that represents the application update schedule.
#[derive(ScheduleLabel)]
pub struct Update;

/// A structure representing an application.
///
/// This encapsulates all convenience wrappers around wgpu as well as a winit
/// [`Window`] and [`EventLoop`].
#[derive(Default, Clone)]
pub struct App {
  state: Arc<Mutex<State>>,
  scheduler: Arc<Mutex<Scheduler>>,
}

impl App {
  /// Create and start the application loop.
  ///
  /// This method takes ownership of the application so everything must happen
  /// within the  from this point and until the application exits.
  pub fn run(&self) {
    self.run_schedule(Init);

    loop {
      self.run_schedule(Update);
    }
  }

  fn run_schedule<R: ScheduleLabel + 'static>(&self, label: R) {
    if let Ok(mut scheduler) = self.scheduler.try_lock() {
      if let Ok(mut state) = self.state.try_lock() {
        scheduler.run(label, &mut state);
      }
    }
  }

  pub fn add_handler<R: ScheduleLabel + 'static, I, S: Handler + 'static>(
    &mut self,
    label: R,
    handler: impl IntoHandler<I, Handler = S>,
  ) -> &mut Self {
    if let Ok(mut scheduler) = self.scheduler.try_lock() {
      scheduler.add_handler(label, handler);
    }

    self
  }

  pub fn add_state<R: 'static>(&mut self, resource: R) -> &mut Self {
    if let Ok(mut state) = self.state.try_lock() {
      state.add(resource);
    }

    self
  }

  pub fn add_module(&mut self, module: impl Module) -> &mut Self {
    module.build(self);
    self
  }
}
