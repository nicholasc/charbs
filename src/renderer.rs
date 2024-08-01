use crate::{
  app::{App, Module, Update},
  events::EventBus,
  rendering::{RenderContext, RenderFrame},
  state::{Res, ResMut},
  window::{Render, Resized},
};

pub struct RendererModule;

impl Module for RendererModule {
  fn build(&self, app: &mut App) {
    app
      .add_handler(Update, Self::resize)
      .add_handler(Render, Self::render);
  }
}

impl RendererModule {
  pub fn resize(mut event_bus: ResMut<EventBus>, ctx: ResMut<RenderContext>) {
    for Resized { width, height } in event_bus.read::<Resized>() {
      ctx.resize(width, height);
    }
  }

  pub fn render(ctx: Res<RenderContext>) {
    let mut frame = RenderFrame::new(ctx.device(), ctx.surface());

    frame.clear(wgpu::Color {
      r: 0.1,
      g: 0.2,
      b: 0.1,
      a: 1.0,
    });

    frame.finish(ctx.queue());
  }
}
