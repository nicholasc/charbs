use crate::buffer::Buffer;

use bytemuck::{Pod, Zeroable};

// A structure that represents a single Vertex buffer for the gpu.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
  position: [f32; 2],
  uv: [f32; 2],
}

// TODO: Feels like this should move to a separate file or module.
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

/// A trait representing a geometry that can be rendered.
pub trait Geometry {
  /// Returns a reference to the geometry's vertex buffer.
  fn vertex_buffer(&self) -> &Buffer<Vertex>;

  /// Returns a reference to the geometry's vertex index layout.
  fn index_buffer(&self) -> &Buffer<u16>;
}

/// A structure representing a triangle geometry.
pub struct Triangle {
  vertex_buffer: Buffer<Vertex>,
  index_buffer: Buffer<u16>,
}

impl Triangle {
  /// A constant describing the indices for the triangle.
  const INDICES: [u16; 3] = [0, 1, 2];

  /// Create a new instance of a two-dimensional [`Triangle`].
  ///
  /// # Arguments
  ///
  /// * `device` - The GPU device for creating the buffers.
  /// * `width` - The width of the triangle.
  /// * `height` - The height of the triangle.
  ///
  /// * `->` - A new instance of a two-dimensional [`Triangle`] geometry.
  pub fn new(device: &wgpu::Device, width: f32, height: f32) -> Self {
    // Create the vertex buffer for the shader program.
    // TODO: Positions should be adjusted for a centroid at 2/3rd of 1.
    #[rustfmt::skip]
    let vertex_buffer = Buffer::<Vertex>::new_with_data(
      device,
      wgpu::BufferUsages::VERTEX,
      &[
        Vertex { position: [-width, -height], uv: [0.0, 1.0] },
        Vertex { position: [0.0, height], uv: [0.5, 0.0] },
        Vertex { position: [width, -height], uv:[1.0, 1.0] },
      ]
    );

    // Create the buffer for indices
    #[rustfmt::skip]
    let index_buffer = Buffer::<u16>::new_with_data(
      device,
      wgpu::BufferUsages::INDEX,
      &Vec::from(Triangle::INDICES)
    );

    Self {
      vertex_buffer,
      index_buffer,
    }
  }
}

impl Geometry for Triangle {
  /// Returns a reference to the geometry's vertex buffer.
  fn vertex_buffer(&self) -> &Buffer<Vertex> {
    &self.vertex_buffer
  }

  /// Returns a reference to the geometry's vertex index layout.
  fn index_buffer(&self) -> &Buffer<u16> {
    &self.index_buffer
  }
}

/// A structure representing a rectangle geometry.
pub struct Rectangle {
  vertex_buffer: Buffer<Vertex>,
  index_buffer: Buffer<u16>,
}

impl Rectangle {
  /// A constant describing the indices for the rectangle.
  const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

  /// Create a new instance of a two-dimensional [`Rectangle`].
  ///
  /// # Arguments
  ///
  /// * `device` - The GPU device for creating the buffers.
  /// * `width` - The width of the rectangle.
  /// * `height` - The height of the rectangle.
  ///
  /// * `->` - A new instance of a two-dimensional [`Rectangle`] geometry.
  pub fn new(device: &wgpu::Device, width: f32, height: f32) -> Self {
    // Create the vertex buffer for the shader program.
    #[rustfmt::skip]
    let vertex_buffer = Buffer::<Vertex>::new_with_data(
      device,
      wgpu::BufferUsages::VERTEX,
      &[
        Vertex { position: [-width, -height], uv: [0.0, 1.0] },
        Vertex { position: [width, -height], uv: [1.0, 1.0] },
        Vertex { position: [width, height], uv: [1.0, 0.0] },
        Vertex { position: [-width, height], uv: [0.0, 0.0] },
      ]
    );

    // Create the buffer for indices
    #[rustfmt::skip]
    let index_buffer = Buffer::<u16>::new_with_data(
      device,
      wgpu::BufferUsages::INDEX,
      &Vec::from(Rectangle::INDICES)
    );

    Self {
      vertex_buffer,
      index_buffer,
    }
  }
}

impl Geometry for Rectangle {
  /// Returns a reference to the geometry's vertex buffer.
  fn vertex_buffer(&self) -> &Buffer<Vertex> {
    &self.vertex_buffer
  }

  /// Returns a reference to the geometry's vertex index layout.
  fn index_buffer(&self) -> &Buffer<u16> {
    &self.index_buffer
  }
}

/// A structure representing a circle geometry.
pub struct Circle {
  vertex_buffer: Buffer<Vertex>,
  index_buffer: Buffer<u16>,
}

impl Circle {
  /// Creates a new instance of a circle with the given radius and number of
  /// segments.
  ///
  /// # Arguments
  ///
  /// * `device` - The GPU device for creating the buffers.
  /// * `radius` - The radius of the circle.
  /// * `segments` - The number of segments to use when drawing the circle.
  ///
  /// * `->` - A new instance of a two-dimensional [`Circle`] geometry.
  pub fn new(device: &wgpu::Device, radius: f32, segments: u32) -> Self {
    let angle = (360.0 / segments as f32).to_radians();

    // Create empty indices array and the circle middle point.
    let mut indices: Vec<u16> = vec![];
    let mut vertices: Vec<Vertex> = vec![Vertex {
      position: [0.0, 0.0],
      uv: [0.5, 0.5],
    }];

    for i in 0..segments {
      // Calculate the angle for the current segment.
      let current_angle = angle * i as f32;

      // Calculate the x and y coordinates for the current segment and push
      // them to the vertices array.
      vertices.push(Vertex {
        position: [radius * current_angle.cos(), radius * current_angle.sin()],
        uv: [0.5, 0.5],
      });

      // Create the indices for the current segment triangle.
      indices.push(0);
      indices.push(i as u16 + 1);
      indices.push(i as u16 + 2);
    }

    // Update the last index to close the circle.
    *indices.last_mut().unwrap() = 1;

    // Create the vertex buffer.
    #[rustfmt::skip]
    let vertex_buffer = Buffer::<Vertex>::new_with_data(
      device,
      wgpu::BufferUsages::VERTEX,
      &vertices
    );

    // Create and store indices count.
    #[rustfmt::skip]
    let index_buffer = Buffer::<u16>::new_with_data(device,
      wgpu::BufferUsages::INDEX,
      &indices
    );

    Self {
      vertex_buffer,
      index_buffer,
    }
  }
}

impl Geometry for Circle {
  /// Returns a reference to the geometry's vertex buffer.
  fn vertex_buffer(&self) -> &Buffer<Vertex> {
    &self.vertex_buffer
  }

  /// Returns a reference to the geometry's vertex index layout.
  fn index_buffer(&self) -> &Buffer<u16> {
    &self.index_buffer
  }
}
