use crate::{
  app::{App, Init, Module, Update},
  state::{Res, ScheduleLabel},
};

use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::{self, EventLoop},
  window::{Window, WindowAttributes},
};

/// A schedule label that represents when the window is being redrawn.
#[derive(ScheduleLabel)]
struct RedrawRequested;

/// A module that manages a winit window.
pub struct WindowModule;

impl WindowModule {
  /// Runs the winit application.
  ///
  /// # Arguments
  ///
  /// * `app` - The [`App`] to be run.
  fn runner(app: App) {
    let mut window_app = WindowApp::new(app);

    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut window_app).unwrap();
  }
}

impl Module for WindowModule {
  fn build(&self, app: &mut App) {
    app
      .add_handler(RedrawRequested, |window: Res<Window>| {
        window.request_redraw()
      })
      .set_runner(Self::runner);
  }
}

/// A winit application that manages a window.
pub struct WindowApp {
  app: App,
}

impl WindowApp {
  /// Creates a new winit application.
  ///
  /// # Arguments
  ///
  /// * `app` - The internal [`App`] to be run.
  pub fn new(app: App) -> Self {
    WindowApp { app }
  }
}

impl ApplicationHandler<()> for WindowApp {
  fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
    // Create the application window
    self.app.add_state(
      event_loop
        .create_window(WindowAttributes::default())
        .unwrap(),
    );

    // Run the application initialization schedule
    self.app.run_schedule(Init);
  }

  fn window_event(
    &mut self,
    event_loop: &event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    match event {
      WindowEvent::CloseRequested => {
        event_loop.exit();
      }

      WindowEvent::RedrawRequested => {
        // Run the application update schedule
        self.app.run_schedule(Update);

        // Run the application redraw schedule
        self.app.run_schedule(RedrawRequested);
      }

      _ => (),
    }
  }
}
