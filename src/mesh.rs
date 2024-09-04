use crate::{
  materials::Material,
  math::{Circle, Rectangle, Triangle},
  resources::ResourceHandle,
  transform::Transform,
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
  vertices: Vec<Vertex>,
  indices: Vec<u16>,
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
