use crate::{
  app::{App, Module},
  rendering::{RenderContext, RenderFrame},
  state::Res,
  window::Render,
};

pub struct RendererModule;

impl Module for RendererModule {
  fn build(&self, app: &mut App) {
    app.add_handler(Render, Self::render);
  }
}

impl RendererModule {
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
