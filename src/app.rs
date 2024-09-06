use crate::{
  assets::Assets,
  events::EventBus,
  materials::{Material, MeshInstancesToSpawn},
  mesh::MeshInstance,
  state::{Handler, IntoHandler, ResMut, ScheduleLabel, Scheduler, State},
};

use std::sync::{Arc, Mutex};

/// A schedule label that represents the application pre-initialization
/// schedule.
#[derive(ScheduleLabel)]
pub struct PreInit;

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
  app.run_schedule(PreInit);
  app.run_schedule(Init);

  loop {
    app.run_schedule(Update);
    app.run_post_loop();
  }
}

/// A structure representing an application.
///
/// This encapsulates all convenience wrappers around a global application
/// [`State`] and a runtime [`Scheduler`] to execute [`Handler`]s.
pub struct App {
  // TODO: Used to provide Window apps easy access to state.
  // This should be replaced by something more elegant.
  pub(crate) state: Arc<Mutex<State>>,
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
  #[inline]
  pub fn run(&mut self) {
    // Take ownership of the runner function and the application.
    let runner = std::mem::replace(&mut self.runner, default_runner);
    let mut app = std::mem::take(self);

    // Create the initial app state.
    app.add_state(Assets::default());
    app.add_state(Commands::default());
    app.add_state(EventBus::default());

    // Up, up and away!
    (runner)(app);
  }

  /// Sets the runner function for the application.
  ///
  /// # Arguments
  ///
  /// * `runner` - The new runner function to be used by the application.
  ///
  /// * `->` - A mutable reference to the [`App`].
  #[inline]
  pub(crate) fn set_runner(&mut self, runner: RunnerFn) -> &mut Self {
    self.runner = runner;

    self
  }

  /// Internal thread-safe method to run a specific schedule in the
  /// [`Scheduler`].
  ///
  /// # Arguments
  ///
  /// * `label` - The schedule label to run.
  #[inline]
  pub(crate) fn run_schedule<R: ScheduleLabel + 'static>(&self, label: R) {
    if let Ok(mut scheduler) = self.scheduler.try_lock() {
      if let Ok(mut state) = self.state.try_lock() {
        scheduler.run(label, &mut state);

        // Take the state from the commands structure and merge the new state into the existing state.
        let mut new_state = std::mem::take(&mut state.get::<ResMut<Commands>>().state);
        state.merge(&mut new_state);
      }
    }
  }

  /// Runs the post-loop logic for the application.
  ///
  /// Execute all commands queued in the [`Commands`] struct and reset the
  /// [`EventBus`] for the next iteration.
  #[inline]
  pub(crate) fn run_post_loop(&mut self) {
    // Reset the event bus.
    if let Ok(state) = self.state.try_lock() {
      state.get::<ResMut<EventBus>>().clear();
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
  #[inline]
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

  /// Add a structure to the application's [`State`].
  ///
  /// # Arguments
  ///
  /// * `structure` - The object to add to the [`State`].
  ///
  /// * `->` - A mutable reference to the [`App`].
  #[inline]
  pub fn add_state<R: 'static>(&mut self, structure: R) -> &mut Self {
    if let Ok(mut state) = self.state.try_lock() {
      state.add(structure);
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
  #[inline]
  pub fn add_module(&mut self, module: impl Module) -> &mut Self {
    module.configure(self);

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
  fn configure(&self, app: &mut App);
}

/// A structure to store application-specific commands that should be executed
/// at the end of a schedule.
///
/// Because schedules execution locks the application's state, we use a separate
/// structure to store these in the form of commands. Once the schedule execution is
/// completed, the commands are executed and reflected onto the application's state.
#[derive(Default)]
pub struct Commands {
  state: State,
}

impl Commands {
  /// Add a structure to the application's [`State`].
  ///
  /// # Arguments
  ///
  /// * `object` - The object to add to the [`State`].
  ///
  /// * `->` - A mutable reference to the [`Commands`].
  #[inline]
  pub fn add_state<R: 'static>(&mut self, structure: R) -> &mut Self {
    self.state.add(structure);

    self
  }

  /// Spawns a mesh instance with a specific material.
  ///
  /// # Arguments
  ///
  /// * `instance` - The mesh instance to spawn.
  #[inline]
  pub fn spawn<M: Material>(&mut self, instance: MeshInstance<M>) {
    // Add a mesh to spawn container to state if there is none.
    if !self.state.has::<MeshInstancesToSpawn<M>>() {
      self.state.add(MeshInstancesToSpawn::<M>::default());
    }

    // Push the instance into the state.
    self
      .state
      .get::<ResMut<MeshInstancesToSpawn<M>>>()
      .push(instance);
  }
}
