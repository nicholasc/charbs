use std::borrow::Cow;

/// A structure that encapsulates a vertex shader, a fragment shader and
/// bindings to the uniforms they rely on.
pub struct Shader {
  inner: wgpu::ShaderModule,
}

impl Eq for Shader {}

impl PartialEq for Shader {
  fn eq(&self, other: &Self) -> bool {
    self.inner.global_id() == other.inner.global_id()
  }
}

impl Shader {
  /// Creates a new shader program on the gpu.
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to use when creating the shader program.
  /// * `source` - The [`Shader`] program source code.
  ///
  /// * `->` - A new [`Shader`] instance ready to be used.
  pub fn new(device: &wgpu::Device, source: &str) -> Self {
    // Create the shader module
    let inner = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source)),
    });

    Self { inner }
  }

  /// Returns a reference to the inner wgpu shader module.
  pub fn inner(&self) -> &wgpu::ShaderModule {
    &self.inner
  }
}
