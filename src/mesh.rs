use crate::{
  assets::Assets,
  binding::{BindGroup, Uniform},
  buffer::Buffer,
  materials::Material,
  math::{Circle, Rectangle, Triangle},
  prelude::RenderContext,
  resources::ResourceHandle,
  transform::{AffineTransform, Transform},
};

use bytemuck::{Pod, Zeroable};

// A structure that represents a single Vertex buffer for the gpu.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
  position: [f32; 2],
  uv: [f32; 2],
}

impl Vertex {
  /// Returns a static description of the vertex buffer layout.
  pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttribute {
          format: wgpu::VertexFormat::Float32x2,
          offset: 0,
          shader_location: 0,
        },
        wgpu::VertexAttribute {
          offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float32x2,
        },
      ],
    }
  }
}

/// A trait representing a mesh geometry that can be rendered.
pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>,
}

impl From<Triangle> for Mesh {
  fn from(value: Triangle) -> Self {
    // Create the vertex buffer for the shader program.
    // TODO: Positions should be adjusted for a centroid at 2/3rd of 1.
    #[rustfmt::skip]
    let vertices = vec![
      Vertex { position: [-value.width, -value.height], uv: [0.0, 1.0] },
      Vertex { position: [0.0, value.height], uv: [0.5, 0.0] },
      Vertex { position: [value.width, -value.height], uv:[1.0, 1.0] },
    ];

    // Create the indices
    let indices = vec![0, 1, 2];

    Self { vertices, indices }
  }
}

impl From<Rectangle> for Mesh {
  fn from(value: Rectangle) -> Self {
    #[rustfmt::skip]
    // Create the vertex buffer for the shader program.
    let vertices = vec![
      Vertex { position: [-value.width, -value.height], uv: [0.0, 1.0] },
      Vertex { position: [ value.width, -value.height], uv: [1.0, 1.0] },
      Vertex { position: [ value.width,  value.height], uv: [1.0, 0.0] },
      Vertex { position: [-value.width,  value.height], uv: [0.0, 0.0] },
    ];

    // Create the buffer for indices
    let indices = vec![0, 1, 2, 0, 2, 3];

    Self { vertices, indices }
  }
}

impl From<Circle> for Mesh {
  fn from(value: Circle) -> Self {
    let angle = (360.0 / value.segments as f32).to_radians();

    // Create empty indices array and the circle middle point.
    let mut indices: Vec<u16> = vec![];
    let mut vertices: Vec<Vertex> = vec![Vertex {
      position: [0.0, 0.0],
      uv: [0.5, 0.5],
    }];

    for i in 0..value.segments {
      // Calculate the angle for the current segment.
      let current_angle = angle * i as f32;

      // Calculate the x and y coordinates for the current segment and push
      // them to the vertices array.
      vertices.push(Vertex {
        position: [
          value.radius * current_angle.cos(),
          value.radius * current_angle.sin(),
        ],
        uv: [0.5, 0.5],
      });

      // Create the indices for the current segment triangle.
      indices.push(0);
      indices.push(i as u16 + 1);
      indices.push(i as u16 + 2);
    }

    // Update the last index to close the circle.
    *indices.last_mut().unwrap() = 1;

    Self { vertices, indices }
  }
}

pub struct MeshInstance<M: Material> {
  pub mesh: ResourceHandle<Mesh>,
  pub material: ResourceHandle<M>,
  pub transform: Transform,
  // transform uniform should be here ?
}

pub struct GPUMesh<M: Material> {
  pub mesh: ResourceHandle<Mesh>,
  pub material: ResourceHandle<M>,
  pub pipeline: wgpu::RenderPipeline,
  pub transform_uniform: Uniform<AffineTransform>,
  pub bind_group: BindGroup,

  pub vertex_buffer: Buffer<Vertex>,
  pub index_buffer: Buffer<u16>,
}

pub trait Meshable {
  fn prepare(&self, ctx: RenderContext, assets: &Assets);
}

impl<M: Material> Meshable for MeshInstance<M> {
  fn prepare(&self, ctx: RenderContext, mut assets: &Assets) {
    // let device = ctx.device();
    // let surface = ctx.surface();
    // let adapter = ctx.adapter();

    // let transform_uniform = Uniform::new(device,
    // AffineTransform::from(self.transform)); let bind_group =
    // BindGroup::new(device, vec![&transform_uniform]);

    // let material = materials.get(&instance.material).unwrap();
    // let shader_source = assets.get(M::shader());
    // let shader = Shader::new(device, shader_source.as_ref());

    // // Create the pipeline layout for the mesh
    // let pipeline_layout =
    //   device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    //     label: None,
    //     bind_group_layouts: &[bind_group.layout(),
    // material.bind_group().layout()],     push_constant_ranges: &[],
    //   });

    // // Create the vertex state
    // let vertex = wgpu::VertexState {
    //   entry_point: "vertex_main",
    //   module: shader.inner(),
    //   buffers: &[Vertex::buffer_layout()],
    //   compilation_options: wgpu::PipelineCompilationOptions::default(),
    // };

    // // Create the targets for the fragment
    // // TODO: Should most likely be configurable in a material description?
    // let targets = [Some(wgpu::ColorTargetState {
    //   format: *surface.get_capabilities(adapter).formats.first().unwrap(),
    //   blend: Some(wgpu::BlendState::ALPHA_BLENDING),
    //   write_mask: wgpu::ColorWrites::ALL,
    // })];

    // // Create the fragment state
    // let fragment = Some(wgpu::FragmentState {
    //   entry_point: "fragment_main",
    //   module: shader.inner(),
    //   targets: &targets,
    //   compilation_options: wgpu::PipelineCompilationOptions::default(),
    // });

    // // Create the render pipeline using the layout and the material
    // let pipeline =
    // device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    //   label: None,
    //   layout: Some(&pipeline_layout),
    //   vertex,
    //   fragment,
    //   multiview: None,
    //   depth_stencil: None,
    //   cache: None,
    //   primitive: wgpu::PrimitiveState::default(),
    //   multisample: wgpu::MultisampleState::default(),
    // });
  }
}

impl<M: Material> Into<Box<dyn Meshable>> for MeshInstance<M> {
  fn into(self) -> Box<dyn Meshable> {
    Box::new(self)
  }
}
