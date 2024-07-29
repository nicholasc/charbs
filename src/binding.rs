use crate::buffer::Buffer;

use encase::{internal::WriteInto, ShaderType, UniformBuffer};

/// A trait suited for structures designed to contain a wgpu binding.
///
/// Useful for defining higher-level bindings and offering methods to streamline
/// the creation of the appropriate wgpu structure for a group.
pub trait Binding {
  /// Provides the wgpu layout entry for the binding.
  ///
  /// # Arguments
  ///
  /// * `index`: The index on which to assign the binding.
  ///
  /// * `->` - A [`wgpu::BindGroupLayoutEntry`] with the specified index.
  fn layout(&self, index: u32) -> wgpu::BindGroupLayoutEntry;

  /// Provides the wgpu group entry for the binding.
  ///
  /// # Arguments
  ///
  /// * `index`: The index on which to assign the binding.
  ///
  /// * `->` - A [`wgpu::BindGroupEntry`] with the specified index.
  fn entry(&self, index: u32) -> wgpu::BindGroupEntry;
}

/// A structure to encapsulate a [`wgpu::BindGroup`] &
/// [`wgpu::BindGroupLayout`].
///
/// This structure generates a wgpu bind group and layout from an array of
/// abstract [`Binding`] implementation. The bindings indices will reflect the
/// position of the binding in the array passed to the `new` function.
#[derive(Debug)]
pub struct BindGroup {
  group: wgpu::BindGroup,
  layout: wgpu::BindGroupLayout,
}

impl BindGroup {
  /// Creates a new bind group based a list of abstract [`Binding`].
  ///
  /// The bindings indices will reflect the position of the binding in the array
  /// passed to the function.
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to which the group will be bound.
  /// * `bindings` - A vector of [`Binding`] from which to create the group.
  ///
  /// * `->` - A [`BindGroup`] with the specified bindings.
  pub fn new(device: &wgpu::Device, bindings: Vec<&dyn Binding>) -> Self {
    // Create the bind group layout
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("Nabe::Material::Layout"),
      entries: &bindings
        .iter()
        .enumerate()
        .map(|(i, u)| u.layout(i as u32))
        .collect::<Vec<wgpu::BindGroupLayoutEntry>>(),
    });

    // Create the bind group using the layout and buffer
    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("Nabe::Material::BindGroup"),
      layout: &layout,
      entries: &bindings
        .iter()
        .enumerate()
        .map(|(i, u)| u.entry(i as u32))
        .collect::<Vec<wgpu::BindGroupEntry<'_>>>(),
    });

    Self { group, layout }
  }

  /// Returns a read-only reference to the wgpu bind group.
  pub fn get(&self) -> &wgpu::BindGroup {
    &self.group
  }

  /// Returns a read-only reference to the wgpu bind group layout.
  pub fn layout(&self) -> &wgpu::BindGroupLayout {
    &self.layout
  }
}

/// An implementation of a wgpu binding representing a uniform.
pub struct Uniform<T: ShaderType + WriteInto> {
  data: T,
  buffer: Buffer<u8>,
  uniform_buffer: UniformBuffer<Vec<u8>>,
}

impl<T: ShaderType + WriteInto> Uniform<T> {
  /// Create a new uniform from abstract data.
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to which the uniform will be bound.
  /// * `data` - The abstract user data that defines the uniform.
  ///
  /// * `->` - A new instance of [`Uniform`] with the specified data.
  pub fn new(device: &wgpu::Device, data: T) -> Self {
    let mut uniform_buffer = UniformBuffer::new(Vec::new());
    uniform_buffer.write(&data).unwrap();

    // Create a new buffer and write the initial data to it
    let buffer = Buffer::new_with_data(
      device,
      wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      uniform_buffer.as_ref(),
    );

    Self {
      data,
      buffer,
      uniform_buffer,
    }
  }

  /// Returns a read-only reference to the uniform data.
  pub fn get(&self) -> &T {
    &self.data
  }

  /// Update internal data and write it to the wgpu buffer.
  ///
  /// # Arguments
  ///
  /// * `queue` - The [`wgpu::Queue`] to which the buffer will be bound.
  /// * `update` - A closure that updates a mutable reference to the uniform
  ///   data.
  pub fn update(&mut self, queue: &wgpu::Queue, update: impl FnOnce(&mut T)) {
    update(&mut self.data);

    if self.uniform_buffer.write(&self.data).is_ok() {
      self.buffer.write(queue, self.uniform_buffer.as_ref())
    }
  }
}

/// Actual implementation of the binding trait.
impl<T: ShaderType + WriteInto> Binding for Uniform<T> {
  fn layout(&self, index: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
      binding: index,
      visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
      ty: wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: wgpu::BufferSize::new(self.buffer.inner().size()),
      },
      count: None,
    }
  }

  fn entry(&self, index: u32) -> wgpu::BindGroupEntry {
    wgpu::BindGroupEntry {
      binding: index,
      resource: self.buffer.inner().as_entire_binding(),
    }
  }
}

// TODO : Write Storage structure
