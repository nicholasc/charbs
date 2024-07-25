use bytemuck::Pod;

use wgpu::util::DeviceExt;

/// A structure to encapsulate a [`wgpu::Buffer`].
pub struct Buffer<T: Copy + Pod> {
  len: usize,
  inner: wgpu::Buffer,
  marker: std::marker::PhantomData<T>,
}

impl<T: Copy + Pod> Buffer<T> {
  /// Creates a empty buffer with a specified usage and size.
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to which the buffer will be bound.
  /// * `len` - The length of the buffer.
  /// * `usage` - The buffer usage represented by a [`wgpu::BufferUsages`].
  /// * `->` - A [`Buffer`] with the specified length and usage.
  pub fn new(device: &wgpu::Device, len: usize, usage: wgpu::BufferUsages) -> Self {
    // Define buffer descriptor
    let descriptor = &wgpu::BufferDescriptor {
      label: None,
      size: len as u64 * std::mem::size_of::<T>() as u64,
      usage,
      mapped_at_creation: false,
    };

    // Create the wgpu buffer
    let inner = device.create_buffer(descriptor);

    Self {
      len,
      inner,
      marker: std::marker::PhantomData,
    }
  }

  /// Creates a new buffer with a specified usage and bytes.
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to which the buffer will be bound.
  /// * `usage` - The buffer usage represented by a [`wgpu::BufferUsages`].
  /// * `data` - An array of data to initialize the buffer with.
  /// * `->` - A [`Buffer`] with the specified length and usage.
  pub fn new_with_data(
    device: &wgpu::Device,
    usage: wgpu::BufferUsages,
    data: &[T],
  ) -> Self {
    let contents = bytemuck::cast_slice(data);

    // Define buffer descriptor
    let descriptor = &wgpu::util::BufferInitDescriptor {
      label: Some("Buffer"),
      contents,
      usage,
    };

    // Create the wgpu buffer
    let inner = device.create_buffer_init(descriptor);

    Self {
      inner,
      len: data.len(),
      marker: std::marker::PhantomData,
    }
  }

  /// Writes a data array at the beginning of the buffer.
  ///
  /// # Arguments
  ///
  /// * `queue` - The [`wgpu::Queue`] to which the buffer will be bound.
  /// * `data` - An array of data to write to the buffer.
  pub fn write(&self, queue: &wgpu::Queue, data: &[T]) {
    self.write_with_offset(queue, data, 0);
  }

  /// Writes a byte array to the buffer at a specific offset.
  ///
  /// # Arguments
  ///
  /// * `queue` - The [`wgpu::Queue`] to which the buffer will be bound.
  /// * `data` - An array of data to write to the buffer.
  /// * `offset` - The offset at which we should start writing.
  pub fn write_with_offset(&self, queue: &wgpu::Queue, data: &[T], offset: usize) {
    if !data.is_empty() {
      queue.write_buffer(
        &self.inner,
        offset as u64 * std::mem::size_of::<T>() as u64,
        bytemuck::cast_slice(data),
      );
    }
  }

  /// Returns the length of the buffer.
  pub fn len(&self) -> usize {
    self.len
  }

  /// Returns true if the buffer is empty.
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// Returns a read-only reference to the underlying wgpu buffer.
  pub fn inner(&self) -> &wgpu::Buffer {
    &self.inner
  }
}
