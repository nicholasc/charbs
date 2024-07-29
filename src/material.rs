use crate::{
  binding::{BindGroup, Binding, Uniform},
  texture::Texture,
};
use encase::ShaderType;
use std::borrow::Cow;

/// A description structure for creating a shader program.
pub struct ShaderDescriptor<'a> {
  /// The shader program source code.
  pub source: &'a str,

  /// An array of all user-defined uniforms to bind.
  pub uniforms: Vec<&'a dyn Binding>,
}

/// A structure that encapsulates a vertex shader, a fragment shader and
/// bindings to the uniforms they rely on.
pub struct Shader {
  /// A shader vertex and fragment modules
  vertex: wgpu::ShaderModule,
  fragment: wgpu::ShaderModule,

  /// Bind group for the uniforms
  bind_group: BindGroup,
}

impl Shader {
  /// Creates a new shader program for usage in a material.
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to use when creating the shader program.
  /// * `desc` - The [`ShaderDescriptor`] used to configure the shader program.
  ///
  /// * `->` - A new [`Shader`] instance ready to be used in a material.
  pub fn new(device: &wgpu::Device, desc: ShaderDescriptor) -> Self {
    let ShaderDescriptor { source, uniforms } = desc;

    // Create the vertex shader module
    let vertex = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source)),
    });

    // Create the fragment shader module
    let fragment = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source)),
    });

    // Create the bind group for the uniforms
    let bind_group = BindGroup::new(device, uniforms);

    Self {
      vertex,
      fragment,

      bind_group,
    }
  }

  /// Returns a read-only reference to the vertex shader module.
  pub fn vertex(&self) -> &wgpu::ShaderModule {
    &self.vertex
  }

  /// Returns a read-only reference to the fragment shader module.
  pub fn fragment(&self) -> &wgpu::ShaderModule {
    &self.fragment
  }

  /// Returns a read-only reference to the uniforms bind group.
  pub fn bind_group(&self) -> &BindGroup {
    &self.bind_group
  }
}

/// A trait that represents a material.
pub trait Material {
  /// Returns a reference-counted instance to the shader module.
  fn shader(&self) -> &Shader;
}

// TODO: Move to a specific namespace? and document
#[rustfmt::skip]
#[derive(ShaderType)]
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32, }

/// An implementation of an actual material that applies a color to a mesh.
pub struct ColorMaterial {
  /// Internal shader module.
  shader: Shader,

  /// The color uniform used to change the material's color.
  color: Uniform<Color>,
}

impl ColorMaterial {
  // TODO: Move these to the Color struct and use it to build the material
  // instead.
  /// Creates a new color material from red, green and blue channels. Expected
  /// color values are shader compatible (between 0.0 and 1.0).
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

    let shader = Shader::new(
      device,
      ShaderDescriptor {
        source: include_str!("shaders/color.wgsl"),
        uniforms: vec![&color],
      },
    );

    Self { color, shader }
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
  /// Returns a reference-counted instance to the shader module.
  fn shader(&self) -> &Shader {
    &self.shader
  }
}

/// An implementation of an actual material that applies a texture to a mesh.
pub struct TextureMaterial {
  /// Internal shader module.
  shader: Shader,
}

impl TextureMaterial {
  /// Creates a new [`TextureMaterial`].
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to use when creating the material.
  /// * `texture` - The texture to use as a material.
  ///
  /// * `->` - A new [`TextureMaterial`] instance ready to be used in a mesh.
  pub fn new<P>(device: &wgpu::Device, texture: Texture) -> Self
  where
    P: AsRef<std::path::Path>,
  {
    let shader = Shader::new(
      device,
      ShaderDescriptor {
        source: include_str!("shaders/texture.wgsl"),
        uniforms: vec![texture.view(), texture.sampler()],
      },
    );

    Self { shader }
  }
}

impl Material for TextureMaterial {
  /// Returns a reference-counted instance to the shader module.
  fn shader(&self) -> &Shader {
    &self.shader
  }
}
