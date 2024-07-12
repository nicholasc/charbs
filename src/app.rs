use crate::state::{Handler, IntoHandler, ScheduleLabel, Scheduler, State};

use std::sync::{Arc, Mutex};

/// A schedule label that represents the application initialization schedule.
#[derive(ScheduleLabel)]
pub struct Init;

/// A schedule label that represents the application update schedule.
#[derive(ScheduleLabel)]
pub struct Update;

/// A structure representing an application.
///
/// This encapsulates all convenience wrappers around a global application
/// [`State`] and a runtime [`Scheduler`] to execute [`Handler`]s.
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

  /// Internal thread-safe method to run a specific schedule in the
  /// [`Scheduler`].
  ///
  /// # Arguments
  ///
  /// * `label` - The schedule label to run.
  fn run_schedule<R: ScheduleLabel + 'static>(&self, label: R) {
    if let Ok(mut scheduler) = self.scheduler.try_lock() {
      if let Ok(mut state) = self.state.try_lock() {
        scheduler.run(label, &mut state);
      }
    }
  }

  /// Add a [`Handler`] to a specfic schedule in the application's
  /// [`Scheduler`].
  ///
  /// # Arguments
  ///
  /// * `label` - The [`ScheduleLabel`] to add the [`Handler`] to.
  /// * `handler` - The [`Handler`] to add to the schedule.
  /// * `->` - A reference to the application.
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

  /// Add a resource to the application's [`State`].
  ///
  /// # Arguments
  ///
  /// * `resource` - The resource to add to the [`State`].
  /// * `->` - A reference to the [`App`].
  pub fn add_state<R: 'static>(&mut self, resource: R) -> &mut Self {
    if let Ok(mut state) = self.state.try_lock() {
      state.add(resource);
    }

    self
  }

  /// Add a [`Module`] to the application.
  ///
  /// # Arguments
  ///
  /// * `module` - The [`Module`] to add to the application.
  /// * `->` - A reference to the [`App`].
  pub fn add_module(&mut self, module: impl Module) -> &mut Self {
    module.build(self);
    self
  }
}

/// A trait for building modules that integrate with the application.
pub trait Module {
  /// Builds module dependencies into the application.
  ///
  /// # Arguments
  ///
  /// * `app` - A mutable reference to the application.
  fn build(&self, app: &mut App);
}
