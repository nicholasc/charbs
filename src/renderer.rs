use crate::{
  app::{App, Commands, Init, Module, Update},
  camera::Camera,
  events::EventBus,
  rendering::{RenderContext, RenderFrame},
  state::{Res, ResMut},
  window::{Render, Window, WindowResized},
};

/// A module that manages a renderer
pub struct RendererModule;

impl Module for RendererModule {
  fn build(&self, app: &mut App) {
    app
      .add_handler(Init, Self::init)
      .add_handler(Update, Self::resize)
      .add_handler(Render, Self::render);
  }
}

impl RendererModule {
  /// Initializes the basic structures and resources required for rendering.
  ///
  /// # Arguments
  ///
  /// * `commands` - A mutable reference to the [`Commands`] dispatcher.
  /// * `ctx` - The rendering context used to initialize resources.
  /// * `window` - The main window.
  pub fn init(
    mut commands: ResMut<Commands>,
    ctx: Res<RenderContext>,
    window: Res<Window>,
  ) {
    let inner_size = window.inner_size();

    commands.add_state(Camera::new(
      ctx.device(),
      inner_size.width as f32 / inner_size.height as f32,
      1.0,
    ));
  }

  /// Resizes the rendering context when the window is resized.
  ///
  /// # Arguments
  ///
  /// * `event_bus` - The event bus to read window resized events.
  /// * `ctx` - The rendering context to resize.
  pub fn resize(mut event_bus: ResMut<EventBus>, ctx: ResMut<RenderContext>) {
    for WindowResized { width, height } in event_bus.read::<WindowResized>() {
      ctx.resize(width, height);
    }
  }

  /// Renders a frame using the rendering context.
  ///
  /// # Arguments
  ///
  /// * `ctx` - The rendering context to render the frame.
  pub fn render(ctx: Res<RenderContext>) {
    let mut frame = RenderFrame::new(ctx.device(), ctx.surface());

    frame.clear(wgpu::Color {
      r: 0.1,
      g: 0.2,
      b: 0.0,
      a: 1.0,
    });

    frame.finish(ctx.queue());
  }
}
