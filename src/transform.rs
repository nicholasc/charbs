use encase::ShaderType;

use glam::{Affine2, Vec2};

/// Internal shader type structure to represent an affine transformation.
#[derive(ShaderType)]
pub(crate) struct AffineTransform {
  // Represents the rotation and scale transformations.
  matrix2: glam::Mat2,

  // Represents the translation transformation (position).
  translation: glam::Vec2,
}

impl From<Transform> for AffineTransform {
  /// Creates a new affine transformation from a transform.
  ///
  /// # Arguments
  ///
  /// * `transform` - Transform from which to create a new affine transformation
  ///
  /// * `->` A new affine transformation created from the given transform.
  fn from(value: Transform) -> Self {
    let affine =
      Affine2::from_scale_angle_translation(value.scale, value.rotation, value.position);

    AffineTransform {
      matrix2: affine.matrix2,
      translation: affine.translation,
    }
  }
}

/// A structure that represents a transform which describes the position,
/// rotation and scale of an object. Usually used in structures that have world
/// positions such as a [`crate::mesh::Mesh`] or a [`crate::camera::Camera`].
/// TODO: Implement look_at method.
#[derive(Debug, Copy, Clone)]
pub struct Transform {
  /// The position of the transform.
  pub position: Vec2,

  /// The rotation in radiant.
  pub rotation: f32,

  /// The scale of the transform.
  pub scale: Vec2,
}

// Default transform properties.
impl Default for Transform {
  fn default() -> Self {
    Self {
      position: Vec2::new(0.0, 0.0),
      rotation: 0.0,
      scale: Vec2::new(1.0, 1.0),
    }
  }
}
