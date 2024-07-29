use crate::{
  binding::Uniform,
  transform::{AffineTransform, Transform},
};

use encase::ShaderType;

/// A structure representing the data from a camera that is sent to a shader.
#[derive(ShaderType)]
pub struct CameraUniform {
  // TODO: Change zoom to bounds.
  zoom: f32,
  #[align(16)]
  transform: AffineTransform,
}

/// A structure representing the camera through which the users will see the
/// objects in the scene.
pub struct Camera {
  pub transform: Transform,
  uniform: Uniform<CameraUniform>,
}

impl Camera {
  /// Creates a new camera with a specific aspect ratio and zoom.
  ///
  /// # Arguments
  ///
  /// * `device` - The GPU device for creating the camera's uniform buffer.
  /// * `aspect_ratio` - The current screen aspect ratio.
  /// * `zoom` - The initial zoom level at which we initialize the camera.
  ///
  /// * `->` - A new [`Camera`] with the specified aspect ratio and zoom level.
  pub fn new(device: &wgpu::Device, aspect_ratio: f32, zoom: f32) -> Self {
    let mut transform = Transform::default();
    transform.scale.x = 1.0;
    transform.scale.y = aspect_ratio;

    let uniform = Uniform::new(
      device,
      CameraUniform {
        zoom,
        transform: AffineTransform::from(transform),
      },
    );

    Self { transform, uniform }
  }

  /// Returns a read-only reference to the camera aspect ratio.
  pub fn aspect_ratio(&self) -> f32 {
    self.transform.scale.y
  }

  /// Updates the camera's aspect ratio to a given value.
  ///
  /// # Arguments
  ///
  /// * `aspect_ratio` - The new screen aspect ratio.
  pub fn set_aspect(&mut self, aspect_ratio: f32) {
    self.transform.scale.x = 1.0;
    self.transform.scale.y = aspect_ratio;
  }

  /// Returns a read-only reference to the camera zoom level.
  pub fn zoom(&self) -> f32 {
    self.uniform.get().zoom
  }

  /// Updates the camera's zoom to a given value.
  ///
  /// # Arguments
  ///
  /// * `zoom` - The new camera zoom level.
  pub fn set_zoom(&mut self, _zoom: f32) {
    // TODO: Figure out how to pass queue
    // self.uniform.update(|u| u.zoom = zoom);
  }

  /// Updates the camera transform uniform buffer.
  /// TODO: This should not exist and transform should be a uniform somehow.
  pub fn update(&mut self, queue: &wgpu::Queue) {
    self.uniform.update(queue, |u| {
      u.transform = AffineTransform::from(self.transform)
    });
  }

  /// Returns a read-only reference to the camera's uniform buffer.
  pub fn uniform(&self) -> &Uniform<CameraUniform> {
    &self.uniform
  }
}
