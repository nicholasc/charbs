use crate::state::{Handler, IntoHandler, ResMut, ScheduleLabel, Scheduler, State};

use std::sync::{Arc, Mutex};

/// A schedule label that represents the application initialization schedule.
#[derive(ScheduleLabel)]
pub struct Init;

/// A schedule label that represents the application update schedule.
#[derive(ScheduleLabel)]
pub struct Update;

/// A type alias for a function that takes an application and runs it.
type RunnerFn = fn(App);

/// A default runner function that initializes the application and runs it in a
/// loop.
///
/// # Arguments
///
/// * `app` - The application to be initialized and runned.
fn default_runner(mut app: App) {
  app.run_schedule(Init);

  loop {
    app.run_schedule(Update);
    app.execute_commands();
  }
}

/// A structure representing an application.
///
/// This encapsulates all convenience wrappers around a global application
/// [`State`] and a runtime [`Scheduler`] to execute [`Handler`]s.
pub struct App {
  state: Arc<Mutex<State>>,
  scheduler: Arc<Mutex<Scheduler>>,
  runner: RunnerFn,
}

/// Implement the [`Default`] trait for the [`App`] struct.
impl Default for App {
  fn default() -> Self {
    Self {
      state: Default::default(),
      scheduler: Default::default(),
      runner: default_runner,
    }
  }
}

impl App {
  /// Runs the application using the provided runner function.
  pub fn run(&mut self) {
    // Take ownership of the runner function and the application.
    let runner = std::mem::replace(&mut self.runner, default_runner);
    let mut app = std::mem::take(self);

    // Create the initial commands buffer.
    app.add_resource(Commands::default());

    // Up, up and away!
    (runner)(app);
  }

  /// Sets the runner function for the application.
  ///
  /// # Arguments
  ///
  /// * `runner` - The new runner function to be used by the application.
  pub(crate) fn set_runner(&mut self, runner: RunnerFn) {
    self.runner = runner;
  }

  /// Internal thread-safe method to run a specific schedule in the
  /// [`Scheduler`].
  ///
  /// # Arguments
  ///
  /// * `label` - The schedule label to run.
  pub(crate) fn run_schedule<R: ScheduleLabel + 'static>(&self, label: R) {
    if let Ok(mut scheduler) = self.scheduler.try_lock() {
      if let Ok(mut state) = self.state.try_lock() {
        scheduler.run(label, &mut state);
      }
    }
  }

  /// Execute all commands queued in the [`Commands`] struct.
  pub(crate) fn execute_commands(&mut self) {
    if let Ok(mut state) = self.state.try_lock() {
      // Take the state from the commands structure.
      let mut new_state = std::mem::take(&mut state.get::<ResMut<Commands>>().state);

      // Merge the new state into the existing state.
      state.merge(&mut new_state);
    }
  }

  /// Add a [`Handler`] to a specfic schedule in the application's
  /// [`Scheduler`].
  ///
  /// # Arguments
  ///
  /// * `label` - The [`ScheduleLabel`] to add the [`Handler`] to.
  /// * `handler` - The [`Handler`] to add to the schedule.
  ///
  /// * `->` - A mutable reference to the [`App`].
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
  ///
  /// * `->` - A mutable reference to the [`App`].
  pub fn add_resource<R: 'static>(&mut self, resource: R) -> &mut Self {
    if let Ok(mut state) = self.state.try_lock() {
      state.add_resource(resource);
    }

    self
  }

  /// Add a [`Module`] to the application.
  ///
  /// # Arguments
  ///
  /// * `module` - The [`Module`] to add to the application.
  ///
  /// * `->` - A mutable reference to the [`App`].
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
  /// * `app` - A mutable reference to the [`App`].
  fn build(&self, app: &mut App);
}

#[derive(Default)]
pub struct Commands {
  pub(crate) state: State,
}

impl Commands {
  /// Add a resource to the application's [`State`].
  ///
  /// # Arguments
  ///
  /// * `resource` - The resource to add to the [`State`].
  ///
  /// * `->` - A mutable reference to the [`Commands`].
  pub fn add_resource<R: 'static>(&mut self, resource: R) -> &mut Self {
    self.state.add_resource(resource);

    self
  }
}
