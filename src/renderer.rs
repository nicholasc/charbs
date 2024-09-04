use core::str;

use crate::{
  app::{App, Commands, Init, Module, SubApp, Update},
  assets::Assets,
  binding::{BindGroup, Uniform},
  camera::Camera,
  events::EventBus,
  materials::Material,
  mesh::{MeshInstance, Vertex},
  rendering::{RenderContext, RenderFrame},
  resources::Resources,
  shader::Shader,
  state::{Res, ResMut},
  transform::AffineTransform,
  window::{Render, Window, WindowResized},
};

pub struct Renderer;

// registering a material could allow me to genrate systems that will
// automatically create mesh instance for them.
impl Renderer {
  fn spawn<M: Material>(
    &self,
    ctx: RenderContext,
    mut assets: Assets,
    materials: Resources<M>,
    instance: MeshInstance<M>,
  ) {
    let device = ctx.device();
    let surface = ctx.surface();
    let adapter = ctx.adapter();

    let transform_uniform =
      Uniform::new(device, AffineTransform::from(instance.transform));
    let bind_group = BindGroup::new(device, vec![&transform_uniform]);

    let material = materials.get(&instance.material).unwrap();
    let shader_source = assets.get(M::shader());
    let shader = Shader::new(device, shader_source.as_ref());

    // Create the pipeline layout for the mesh
    let pipeline_layout =
      device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[bind_group.layout(), material.bind_group().layout()],
        push_constant_ranges: &[],
      });

    // Create the vertex state
    let vertex = wgpu::VertexState {
      entry_point: "vertex_main",
      module: shader.inner(),
      buffers: &[Vertex::buffer_layout()],
      compilation_options: wgpu::PipelineCompilationOptions::default(),
    };

    // Create the targets for the fragment
    // TODO: Should most likely be configurable in a material description?
    let targets = [Some(wgpu::ColorTargetState {
      format: *surface.get_capabilities(adapter).formats.first().unwrap(),
      blend: Some(wgpu::BlendState::ALPHA_BLENDING),
      write_mask: wgpu::ColorWrites::ALL,
    })];

    // Create the fragment state
    let fragment = Some(wgpu::FragmentState {
      entry_point: "fragment_main",
      module: shader.inner(),
      targets: &targets,
      compilation_options: wgpu::PipelineCompilationOptions::default(),
    });

    // Create the render pipeline using the layout and the material
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      layout: Some(&pipeline_layout),
      vertex,
      fragment,
      multiview: None,
      depth_stencil: None,
      cache: None,
      primitive: wgpu::PrimitiveState::default(),
      multisample: wgpu::MultisampleState::default(),
    });
  }
}

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
