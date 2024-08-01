use crate::{
  app::{App, Init, Module, Update},
  events::{Event, EventBus},
  state::{Res, ResMut, ScheduleLabel},
};

use std::{any::Any, sync::Arc};

use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::{self, EventLoop},
  window::{Window, WindowAttributes},
};

/// A private schedule label that represents when the window is being redrawn.
#[derive(ScheduleLabel)]
pub(crate) struct Render;

/// An event that represents when the window is resized.
#[derive(Event)]
pub struct WindowResized {
  pub width: u32,
  pub height: u32,
}

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
      .set_runner(Self::runner)
      .add_handler(Init, Self::request_redraw)
      .add_handler(Render, Self::request_redraw);
  }
}

impl WindowModule {
  /// Requests a redraw of the window.
  ///
  /// # Arguments
  ///
  /// * `window` - The window to request a redraw.
  pub fn request_redraw(window: Res<Arc<Window>>) {
    window.request_redraw();
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
  ///
  /// * `->` A new winit application.
  pub fn new(app: App) -> Self {
    WindowApp { app }
  }
}

impl ApplicationHandler<()> for WindowApp {
  fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
    // Create the application window
    self.app.add_resource(Arc::new(
      event_loop
        .create_window(WindowAttributes::default())
        .unwrap(),
    ));

    // Run the application initialization schedule
    self.app.run_schedule(Init);

    // Execute the application commands
    self.app.run_commands();
  }

  fn window_event(
    &mut self,
    event_loop: &event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    match event {
      WindowEvent::Resized(size) => {
        // Dispatch the resize event to the event bus
        if let Ok(state) = self.app.state.try_lock() {
          state.get::<ResMut<EventBus>>().write(WindowResized {
            width: size.width,
            height: size.height,
          });
        }
      }

      WindowEvent::CloseRequested => {
        event_loop.exit();
      }

      WindowEvent::RedrawRequested => {
        // Run the application update schedule
        self.app.run_schedule(Update);

        // Run the application redraw schedule
        self.app.run_schedule(Render);

        // Run the post-loop logic
        self.app.run_post_loop();
      }

      _ => (),
    }
  }
}
