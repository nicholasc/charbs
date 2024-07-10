use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::{self, ControlFlow, EventLoop},
  window::{Window, WindowAttributes},
};

use crate::state::{Res, ScheduleLabel, Scheduler, State};

enum AppEvents {}

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
#[derive(Default)]
pub struct App {
  state: State,
  scheduler: Scheduler,
}

impl App {
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

  pub fn state(&self) -> &State {
    &self.state
  }

  pub fn state_mut(&mut self) -> &mut State {
    &mut self.state
  }

  pub fn scheduler(&self) -> &Scheduler {
    &self.scheduler
  }

  pub fn scheduler_mut(&mut self) -> &mut Scheduler {
    &mut self.scheduler
  }
}

impl ApplicationHandler<AppEvents> for App {
  fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
    // Create the application window
    let window = event_loop
      .create_window(WindowAttributes::default())
      .expect("Failed to create window");

    // Store window within the application state
    self.state.add(window);

    // Initialize the application by calling the init schedule
    self.scheduler.run(Init, &mut self.state);
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
        scheduler.run(Update, state);

        // Immediately request the next frame
        state.get::<Res<Window>>().request_redraw();
      }
      _ => (),
    }
  }
}
