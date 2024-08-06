use crate::{
  app::{App, Init, Module, PreInit, Update},
  events::{Event, EventBus},
  state::{Res, ResMut, ScheduleLabel},
};

use std::{any::Any, sync::Arc};

use winit::{
  application::ApplicationHandler,
  event::{StartCause, WindowEvent},
  event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop},
  window::{Window as WinitWindow, WindowAttributes},
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
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut window_app).unwrap();
  }
}

impl Module for WindowModule {
  fn build(&self, app: &mut App) {
    app.set_runner(Self::runner);
  }
}

pub struct Window {
  inner: Arc<WinitWindow>,
}

impl Window {
  /// Creates a new [`Window`] from an existing [`WinitWindow`].
  ///
  /// # Arguments
  ///
  /// * `window` - The existing [`WinitWindow`] to create the new [`Window`].
  ///
  /// * `->` - A new [`Window`] that wraps the given [`WinitWindow`].
  pub fn new(window: WinitWindow) -> Self {
    Self {
      inner: Arc::new(window),
    }
  }

  /// Returns an atomically reference counted pointer to the inner
  /// [`WinitWindow`] .
  pub fn clone(&self) -> Arc<WinitWindow> {
    self.inner.clone()
  }
}

impl std::ops::Deref for Window {
  type Target = WinitWindow;

  fn deref(&self) -> &Self::Target {
    &self.inner
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
  fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
    // Only request redraw if the application is being polled
    // This avoids jittery redraws when window is being resized or moved
    if cause == StartCause::Poll {
      if let Ok(state) = self.app.state.try_lock() {
        state.get::<Res<Window>>().request_redraw();
      }
    }
  }

  fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
    // Create the application window as a resource
    self.app.add_resource(Window::new(
      event_loop
        .create_window(WindowAttributes::default())
        .unwrap(),
    ));

    // Initialize the application & run pre-init commands
    self.app.run_schedule(PreInit);

    // Initialize the application & run init commands
    self.app.run_schedule(Init);
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
