use crate::{
  app::{App, Init, Module},
  binding::{BindGroup, Uniform},
  prelude::RenderContext,
  resources::{ResourceId, Resources},
  shader::{Shader, ShaderId, Shaders},
  state::{Res, ResMut},
  texture::Texture,
};

use encase::ShaderType;

pub type MaterialId = ResourceId<Box<dyn Material>>;

pub type Materials = Resources<Box<dyn Material>>;

/// A trait that represents a material.
pub trait Material {
  /// Returns the shader id associated with this material.
  ///
  /// # Arguments
  ///
  /// * `->` - The shader id associated with this material.
  fn shader() -> ShaderId
  where
    Self: Sized;

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
  fn shader() -> ShaderId {
    "shaders/color.wgsl".into()
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
  fn shader() -> ShaderId {
    "shaders/texture.wgsl".into()
  }

  fn bind_group(&self) -> &BindGroup {
    &self.bind_group
  }
}

pub struct DefaultMaterials;

impl Module for DefaultMaterials {
  fn build(&self, app: &mut App) {
    app.add_handler(Init, Self::pre_init);
  }
}

impl DefaultMaterials {
  fn pre_init(ctx: Res<RenderContext>, mut shaders: ResMut<Shaders>) {
    shaders.add(
      ColorMaterial::shader(),
      Shader::new(ctx.device(), include_str!("shaders/color.wgsl")),
    );

    shaders.add(
      TextureMaterial::shader(),
      Shader::new(ctx.device(), include_str!("shaders/texture.wgsl")),
    );
  }
}
