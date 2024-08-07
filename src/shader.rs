use std::{borrow::Cow, collections::HashMap};

/// A structure that encapsulates a vertex shader, a fragment shader and
/// bindings to the uniforms they rely on.
pub struct Shader {
  inner: wgpu::ShaderModule,
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
}

#[derive(Default)]
pub struct Shaders<'a> {
  data: HashMap<&'a str, Shader>,
}

impl<'a> Shaders<'a> {
  pub fn add(&mut self, label: &'a str, shader: Shader) {
    self.data.insert(label, shader);
  }
}
