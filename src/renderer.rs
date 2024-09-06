use wgpu::core::global::Global;

use crate::{
  app::{App, Commands, Init, Module, Update},
  binding::BindGroup,
  camera::Camera,
  events::EventBus,
  mesh::Mesh,
  rendering::{RenderContext, RenderFrame},
  resources::Resources,
  state::{Res, ResMut},
  window::{Render, Window, WindowResized},
};

pub(crate) type GlobalBindGroup = BindGroup;

/// A module that manages a renderer
pub struct RendererModule;

impl Module for RendererModule {
  fn build(&self, app: &mut App) {
    app
      .add_state(Resources::<Mesh>::default())
      .add_handler(Init, Self::init)
      .add_handler(Update, Self::resize);
    // .add_handler(Render, Self::render);
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

    let camera = Camera::new(
      ctx.device(),
      inner_size.width as f32 / inner_size.height as f32,
      1.0,
    );

    commands.add_state(GlobalBindGroup::new(ctx.device(), vec![camera.uniform()]));
    commands.add_state(camera);
  }

  /// Resizes the rendering context when the window is resized.
  ///
  /// # Arguments
  ///
  /// * `event_bus` - The event bus to read window resized events.
  /// * `ctx` - The rendering context to resize.
  pub fn resize(
    mut event_bus: ResMut<EventBus>,
    ctx: ResMut<RenderContext>,
    mut camera: ResMut<Camera>,
  ) {
    for WindowResized { width, height } in event_bus.read::<WindowResized>() {
      if width > 0 && height > 0 {
        // Resize wgpu context
        ctx.resize(width, height);

        // Resize camera and update mouse window size
        camera.set_aspect(width as f32 / height as f32);
        camera.update(ctx.queue());
        // TODO: Update mouse window size
      }
    }
  }

  /// Renders a frame using the rendering context.
  ///
  /// # Arguments
  ///
  /// * `ctx` - The rendering context to render the frame.
  pub fn render(ctx: Res<RenderContext>) {
    // let mut frame = RenderFrame::new(ctx.device(), ctx.surface());

    // frame.clear(wgpu::Color {
    //   r: 0.1,
    //   g: 0.2,
    //   b: 0.0,
    //   a: 1.0,
    // });

    // let render_pass = frame.create_render_pass();

    // for (pipeline, bind_group) in mesh_instances_pipelines.iter() {
    //   // Prepare the shader program
    //   render_pass.inner.set_pipeline(&pipeline);
    //   render_pass.inner.set_bind_group(0, self.globals.get(), &[]);
    //   render_pass.inner.set_bind_group(1, bind_group.get(), &[]);
    //   render_pass
    //     .inner
    //     .set_bind_group(2, self.shader.bind_group().get(), &[]);

    //   // Draw the geometry
    //   render_pass
    //     .inner
    //     .set_vertex_buffer(0, self.buffer.inner().slice(..));
    //   render_pass.inner.set_index_buffer(
    //     self.index_buffer.inner().slice(..),
    //     wgpu::IndexFormat::Uint16,
    //   );

    //   render_pass
    //     .inner
    //     .draw_indexed(0..self.index_buffer.len(), 0, 0..1);
    // }

    // frame.finish(ctx.queue());
  }
}
