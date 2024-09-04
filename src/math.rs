pub struct Triangle {
  pub width: f32,
  pub height: f32,
}

impl Triangle {
  /// Create a new instance of a two-dimensional [`Triangle`].
  ///
  /// # Arguments
  ///
  /// * `width` - The width of the triangle.
  /// * `height` - The height of the triangle.
  ///
  /// * `->` - A new instance of a two-dimensional [`Triangle`] geometry.
  #[inline(always)]
  pub const fn new(width: f32, height: f32) -> Self {
    Self { width, height }
  }
}

pub struct Rectangle {
  pub width: f32,
  pub height: f32,
}

impl Rectangle {
  /// Create a new instance of a two-dimensional [`Rectangle`].
  ///
  /// # Arguments
  ///
  /// * `width` - The width of the rectangle.
  /// * `height` - The height of the rectangle.
  ///
  /// * `->` - A new instance of a two-dimensional [`Rectangle`] geometry.
  #[inline(always)]
  pub const fn new(width: f32, height: f32) -> Self {
    Self { width, height }
  }
}

/// A structure representing a circle geometry.
pub struct Circle {
  pub radius: f32,
  pub segments: u32,
}

impl Circle {
  /// Creates a new instance of a circle with the given radius and number of
  /// segments.
  ///
  /// # Arguments
  ///
  /// * `radius` - The radius of the circle.
  /// * `segments` - The number of segments to use when drawing the circle.
  ///
  /// * `->` - A new instance of a two-dimensional [`Circle`] geometry.
  #[inline(always)]
  pub const fn new(radius: f32, segments: u32) -> Self {
    Self { radius, segments }
  }
}
