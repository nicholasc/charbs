use std::sync::Arc;

use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::{self, ControlFlow, EventLoop},
  window::{Window, WindowAttributes},
};

use crate::state::{Res, Scheduler, State};

struct AppEvents {}

/// A structure representing an application.
///
/// This encapsulates all convenience wrappers around wgpu as well as a winit
/// [`Window`] and [`EventLoop`].
pub struct App {
  state: State,
  scheduler: Scheduler,
}

impl App {
  pub fn new() -> Self {
    Self {
      state: State::default(),
      scheduler: Scheduler::default(),
    }
  }

  /// Create and start the application loop.
  ///
  /// This method takes ownership of the application so everything must happen
  /// within the  from this point and until the application exits.
  ///
  /// # Arguments
  ///
  /// * `->` - The result of the application loop.
  pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
    // Create the event loop & run app
    let event_loop = EventLoop::<AppEvents>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut self).map_err(Into::into)
  }

  pub fn scheduler_mut(&mut self) -> &mut Scheduler {
    &mut self.scheduler
  }
}

impl ApplicationHandler<AppEvents> for App {
  fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
    // Create the application window
    let window = Arc::new(
      event_loop
        .create_window(WindowAttributes::default())
        .expect("Failed to create window"),
    );

    self.state.add(window);

    // Initialize the scheduler effectively calling all init runtimes
    self.scheduler.run_init(&mut self.state);
  }

  fn window_event(
    &mut self,
    event_loop: &event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    let Self { scheduler, state } = self;

    match event {
      WindowEvent::CloseRequested => {
        println!("The close button was pressed; stopping");
        event_loop.exit();
      }
      WindowEvent::RedrawRequested => {
        // Run the scheduler update runtimes
        scheduler.run(state);

        // Immediately request the next frame
        state.get::<Res<Arc<Window>>>().request_redraw();
      }
      _ => (),
    }
  }
}
