use crate::{
  binding::{BindGroup, Uniform},
  geometry::{Geometry, GeometryId, Vertex},
  materials::{self, Material, MaterialId},
  prelude::RenderContext,
  shader::Shader,
  transform::{self, AffineTransform, Transform},
};

/// A structure used to describe a mesh when creating.
pub struct MeshDescriptor {
  /// The [`Geometry`] used to give a shape to the mesh.
  pub geometry: GeometryId,

  /// The [`Material`] to apply to the mesh.
  pub material: MaterialId,

  /// The [`Transform`] used to position the mesh.
  pub transform: Transform,
}

/// A structure that encapsulates a [`wgpu::RenderPipeline`] used to render a
/// geometry with a specific shader program.
pub struct Mesh {
  // Mesh transform and affine transformation uniform.
  transform: Transform,
  transform_uniform: Uniform<AffineTransform>,

  // The wgpu pipeline for rendering the mesh.
  pipeline: wgpu::RenderPipeline,

  // Mesh bind groups
  bind_group: BindGroup,
}

impl Mesh {
  /// Creates a new [`Mesh`] from a specific geometry and material.
  ///
  /// # Arguments
  ///
  /// * `ctx` - The [`RenderContext`] to use when creating the [`Mesh`].
  /// * `desc` - The [`MeshDescriptor`] used to configure the [`Mesh`].
  pub fn new(
    ctx: &RenderContext,
    material: &dyn Material,
    shader: &Shader,
    transform: Transform,
  ) -> Self {
    let device = ctx.device();
    let surface = ctx.surface();
    let adapter = ctx.adapter();

    // Create our mesh uniforms
    let transform_uniform = Uniform::new(device, AffineTransform::from(transform));
    let bind_group = BindGroup::new(device, vec![&transform_uniform]);

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

    Mesh {
      transform,

      pipeline,

      bind_group,
      transform_uniform,
    }
  }

  /// Returns a read-only reference to the mesh transform.
  pub fn transform(&self) -> &Transform {
    &self.transform
  }

  /// Returns a mutable reference to the mesh transform.
  pub fn transform_mut(&mut self) -> &mut Transform {
    &mut self.transform
  }
}

impl Renderable for Mesh {
  /// Updates the mesh uniform buffers.
  fn update(&mut self) {
    self
      .transform_uniform
      .update(|u| *u = AffineTransform::from(self.transform));
  }

  /// Renders the current geometry using the pipeline.
  ///
  /// # Arguments
  ///
  /// * `render_pass` - The render pass onto which we render this pipeline.
  fn render<'b>(&'b self, render_pass: &mut wgpu::RenderPass<'b>) {
    // Prepare the shader program
    render_pass.set_pipeline(&self.pipeline);
    render_pass.set_bind_group(0, self.globals.get(), &[]);
    render_pass.set_bind_group(1, self.bind_group.get(), &[]);
    render_pass.set_bind_group(2, self.shader.bind_group().get(), &[]);

    // Draw the geometry
    render_pass.set_vertex_buffer(0, self.buffer.inner().slice(..));
    render_pass.set_index_buffer(
      self.index_buffer.inner().slice(..),
      wgpu::IndexFormat::Uint16,
    );

    render_pass.draw_indexed(0..self.index_buffer.len(), 0, 0..1);
  }
}
