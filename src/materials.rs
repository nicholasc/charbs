use crate::{
  app::{App, Module, Update},
  assets::Assets,
  binding::{BindGroup, Uniform},
  buffer::Buffer,
  mesh::{GPUMesh, Mesh, MeshInstance, Vertex},
  prelude::RenderContext,
  renderer::GlobalBindGroup,
  resources::Resources,
  shader::Shader,
  state::{Res, ResMut},
  texture::Texture,
  transform::AffineTransform,
  window::Render,
};

use encase::ShaderType;

/// A trait that represents a material.
pub trait Material: 'static {
  /// Returns the shader id associated with this material.
  ///
  /// TODO: Would be great to return a ShaderSource that could be built from
  /// a string but also actual source code.
  ///
  /// # Arguments
  ///
  /// * `->` - The shader id associated with this material.
  fn shader() -> &'static str;

  /// Returns the bind group associated with this material.
  ///
  /// TODO: Could eventually be replaced by #[] macro to simplify bindings
  /// definition.
  ///
  /// # Arguments
  ///
  /// * `->` - The bind group associated with this material.
  fn bind_group(&self) -> &BindGroup;
}

// TODO: Move to a specific namespace? and document
#[rustfmt::skip]
#[derive(ShaderType)]
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32, }

/// An implementation of an actual material that applies a color to a mesh.
pub struct ColorMaterial {
  /// The color uniform used to change the material's color.
  color: Uniform<Color>,

  /// Bind group for the uniforms
  bind_group: BindGroup,
}

impl ColorMaterial {
  /// Creates a new color material from red, green and blue channels. Expected
  /// color values are shader compatible (between 0.0 and 1.0).
  ///
  /// TODO: Move these to the Color struct and use it to build the material
  /// instead.
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to use when creating the material.
  /// * `r` - The red channel value
  /// * `g` - The green channel value
  /// * `b` - The blue channel value
  ///
  /// * `->` - A new [`ColorMaterial`] instance ready to be used in a mesh.
  pub fn new(device: &wgpu::Device, r: f32, g: f32, b: f32) -> Self {
    Self::new_with_alpha(device, r, g, b, 1.0)
  }

  /// Creates a new color material from red, green, blue and alpha
  /// (transparency) channels. Expected color values are shader compatible
  /// (between 0.0 and 1.0).
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to use when creating the material.
  /// * `r` - The red channel value
  /// * `g` - The green channel value
  /// * `b` - The blue channel value
  ///
  /// * `->` - A new [`ColorMaterial`] instance ready to be used in a mesh with.
  pub fn new_with_alpha(device: &wgpu::Device, r: f32, g: f32, b: f32, a: f32) -> Self {
    let color = Uniform::new(device, Color { r, g, b, a });
    let bind_group = BindGroup::new(device, vec![&color]);

    Self { color, bind_group }
  }

  /// Returns a read-only reference to the color uniform.
  pub fn color(&self) -> &Uniform<Color> {
    &self.color
  }

  /// Returns mutable reference to the color uniform.
  pub fn color_mut(&mut self) -> &mut Uniform<Color> {
    &mut self.color
  }
}

impl Material for ColorMaterial {
  fn shader() -> &'static str {
    "shaders/color.wgsl"
  }

  fn bind_group(&self) -> &BindGroup {
    &self.bind_group
  }
}

/// An implementation of an actual material that applies a texture to a mesh.
pub struct TextureMaterial {
  bind_group: BindGroup,
}

impl TextureMaterial {
  /// Creates a new [`TextureMaterial`].
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to use when creating the material.
  /// * `texture` - The [`Texture`] to use as a material.
  ///
  /// * `->` - A new [`TextureMaterial`] instance ready to be used in a mesh.
  pub fn new<P>(device: &wgpu::Device, texture: &Texture) -> Self
  where
    P: AsRef<std::path::Path>,
  {
    let bind_group = BindGroup::new(device, vec![texture.view(), texture.sampler()]);

    Self { bind_group }
  }
}

impl Material for TextureMaterial {
  fn shader() -> &'static str {
    "shaders/texture.wgsl"
  }

  fn bind_group(&self) -> &BindGroup {
    &self.bind_group
  }
}

pub struct MaterialModule<M: Material> {
  _marker: std::marker::PhantomData<M>,
}

impl<M: Material> Default for MaterialModule<M> {
  fn default() -> Self {
    Self {
      _marker: std::marker::PhantomData,
    }
  }
}

impl<M: Material> Module for MaterialModule<M> {
  fn configure(&self, app: &mut App) {
    app
      .add_state(Resources::<M>::default())
      .add_state(MeshInstancesToSpawn::<M>::default())
      .add_state(GPUMeshInstances::<M>::default())
      .add_handler(Update, Self::update)
      .add_handler(Render, Self::render);
  }
}

pub(crate) type MeshInstancesToSpawn<M> = Vec<MeshInstance<M>>;

pub(crate) type GPUMeshInstances<M> = Vec<GPUMesh<M>>;

impl<M: Material> MaterialModule<M> {
  fn update(
    ctx: Res<RenderContext>,
    mut assets: ResMut<Assets>,
    meshes: Res<Resources<Mesh>>,
    materials: Res<Resources<M>>,
    mut instances: ResMut<MeshInstancesToSpawn<M>>,
    mut mesh_instances: ResMut<GPUMeshInstances<M>>,
    globals: Res<GlobalBindGroup>,
  ) {
    let device = ctx.device();
    let surface = ctx.surface();
    let adapter = ctx.adapter();

    for instance in instances.drain(..) {
      let transform_uniform =
        Uniform::new(device, AffineTransform::from(instance.transform));
      let bind_group = BindGroup::new(device, vec![&transform_uniform]);

      let mesh = meshes.get(&instance.mesh).unwrap();
      let material = materials.get(&instance.material).unwrap();
      let shader_source = assets.get(M::shader());
      let shader = Shader::new(device, shader_source.as_ref());

      // Create the pipeline layout for the mesh
      let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
          label: None,
          bind_group_layouts: &[
            globals.layout(),
            bind_group.layout(),
            material.bind_group().layout(),
          ],
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

      // TODO: Must store the pipeline and bind group for actual rendering.
      mesh_instances.push(GPUMesh {
        pipeline,
        material: instance.material,
        bind_group,

        vertex_buffer: Buffer::new_with_data(
          device,
          wgpu::BufferUsages::VERTEX,
          &mesh.vertices,
        ),

        index_buffer: Buffer::new_with_data(
          device,
          wgpu::BufferUsages::INDEX,
          &mesh.indices,
        ),
      });
    }
  }

  /// Renders a frame using the rendering context.
  ///
  /// # Arguments
  ///
  /// * `ctx` - The rendering context to render the frame.
  fn render(
    mut ctx: ResMut<RenderContext>,
    materials: Res<Resources<M>>,
    mesh_instances: Res<GPUMeshInstances<M>>,
    globals: Res<GlobalBindGroup>,
  ) {
    if mesh_instances.is_empty() {
      return;
    }

    let mut render_pass = ctx.current_frame_mut().create_render_pass();

    for instance in mesh_instances.iter() {
      let material = materials.get(&instance.material).unwrap();

      // Prepare the shader program
      render_pass.inner.set_pipeline(&instance.pipeline);
      render_pass.inner.set_bind_group(0, globals.get(), &[]);
      render_pass
        .inner
        .set_bind_group(1, instance.bind_group.get(), &[]);
      render_pass
        .inner
        .set_bind_group(2, material.bind_group().get(), &[]);

      // Draw the geometry
      render_pass
        .inner
        .set_vertex_buffer(0, instance.vertex_buffer.inner().slice(..));
      render_pass.inner.set_index_buffer(
        instance.index_buffer.inner().slice(..),
        wgpu::IndexFormat::Uint16,
      );

      render_pass
        .inner
        .draw_indexed(0..instance.index_buffer.len() as u32, 0, 0..1);
    }

    render_pass.finish();
  }
}

pub struct DefaultMaterials;

impl Module for DefaultMaterials {
  fn configure(&self, app: &mut App) {
    app
      .add_module(MaterialModule::<ColorMaterial>::default())
      .add_module(MaterialModule::<TextureMaterial>::default());
  }
}
