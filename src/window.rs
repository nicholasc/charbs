use crate::{
  app::{App, Module, Update},
  state::ResMut,
};

use std::time::Duration;

use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::{self, EventLoop},
  platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
  window::{Window, WindowAttributes},
};

/// A module that manages a winit window.
pub struct WindowModule;

impl Module for WindowModule {
  fn build(&self, app: &mut App) {
    app
      .add_state(EventLoop::new().expect("Failed to create event loop"))
      .add_state(WindowApp::default())
      .add_handler(Update, Self::update);
  }
}

impl WindowModule {
  /// An update function that handles pumping of application events.
  ///
  /// # Arguments
  ///
  /// * `event_loop` - A mutable reference to the winit [`EventLoop`].
  /// * `app` - A mutable reference to the [`WindowApp`].
  fn update(mut event_loop: ResMut<EventLoop<()>>, mut app: ResMut<WindowApp>) {
    let status = event_loop.pump_app_events(Some(Duration::ZERO), &mut *app);

    if let PumpStatus::Exit(exit_code) = status {
      std::process::exit(exit_code);
    }
  }
}

/// A winit application that manages a window.
#[derive(Default)]
pub struct WindowApp {
  window: Option<Window>,
}

impl ApplicationHandler<()> for WindowApp {
  fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
    // Create the application window
    let window = event_loop
      .create_window(WindowAttributes::default())
      .expect("Failed to create window");

    self.window = Some(window);
  }

  fn window_event(
    &mut self,
    event_loop: &event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    match event {
      WindowEvent::CloseRequested => {
        println!("The close button was pressed; stopping");
        event_loop.exit();
      }
      WindowEvent::RedrawRequested => {
        self.window.as_ref().unwrap().request_redraw();
      }
      _ => (),
    }
  }
}
