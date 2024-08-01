use crate::{
  app::{App, Module, Update},
  events::EventBus,
  rendering::{RenderContext, RenderFrame},
  state::{Res, ResMut},
  window::{Render, WindowResized},
};

/// A module that manages a renderer
pub struct RendererModule;

impl Module for RendererModule {
  fn build(&self, app: &mut App) {
    app
      .add_handler(Update, Self::resize)
      .add_handler(Render, Self::render);
  }
}

impl RendererModule {
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
