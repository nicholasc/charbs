use crate::binding::Binding;

use std::path::Path;

/// A structure representing a texture that can be renderer in a shader program.
pub struct Texture {
  view: TextureView,
  sampler: TextureSampler,
}

impl Texture {
  /// Creates a new [`Texture`] from a image located at the specified path.
  ///
  /// # Arguments
  ///
  /// * `device` - The wgpu device to create the texture.
  /// * `queue` - The wgpu queue to submit the texture creation command.
  /// * `path` - The path to the image used to create the texture.
  ///
  /// * `->` A new [`Texture`] created from the image.
  pub fn new<P>(device: &wgpu::Device, queue: &wgpu::Queue, path: P) -> Self
  where
    P: AsRef<Path>,
  {
    // Attempt to open the image from path
    let image = image::open(path).unwrap().to_rgba8();

    // Create texture size from dimensions
    let (width, height) = image.dimensions();
    let texture_size = wgpu::Extent3d {
      width,
      height,
      depth_or_array_layers: 1,
    };

    // Define the texture descriptor
    let desc = &wgpu::TextureDescriptor {
      size: texture_size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      label: None,
      view_formats: &[],
    };

    // Create the actual 2d texture
    let texture = device.create_texture(desc);

    // Define the image copy
    let image_copy = wgpu::ImageCopyTexture {
      texture: &texture,
      mip_level: 0,
      origin: wgpu::Origin3d::ZERO,
      aspect: wgpu::TextureAspect::All,
    };

    // Define the image data layout
    let data_layout = wgpu::ImageDataLayout {
      offset: 0,
      bytes_per_row: Some(4 * width),
      rows_per_image: Some(height),
    };

    // Write image data to the texture
    queue.write_texture(image_copy, &image, data_layout, texture_size);

    // Create the texture view and sampler
    let view = TextureView::new(&texture);

    let sampler = TextureSampler::new(
      device,
      &wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
      },
    );

    Self { view, sampler }
  }

  /// Return a read-only reference to the texture view
  pub fn view(&self) -> &TextureView {
    &self.view
  }

  /// Return a read-only reference to the texture sampler
  pub fn sampler(&self) -> &TextureSampler {
    &self.sampler
  }
}

/// A structure to encapsulate a [`wgpu::TextureView`].
pub struct TextureView {
  inner: wgpu::TextureView,
}

impl TextureView {
  /// Creates a new [`TextureView`]
  ///
  /// # Arguments
  ///
  /// * `texture` - The wgpu texture from which we are creating a view.
  ///
  /// * `->` A new [`TextureView`] from the given texture.
  pub fn new(texture: &wgpu::Texture) -> Self {
    Self {
      inner: texture.create_view(&wgpu::TextureViewDescriptor::default()),
    }
  }
}

/// Implementation of the binding trait to use this in a shader material.
impl Binding for TextureView {
  fn layout(&self, index: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
      binding: index,
      visibility: wgpu::ShaderStages::FRAGMENT,
      ty: wgpu::BindingType::Texture {
        multisampled: false,
        view_dimension: wgpu::TextureViewDimension::D2,
        sample_type: wgpu::TextureSampleType::Float { filterable: true },
      },
      count: None,
    }
  }

  fn entry(&self, index: u32) -> wgpu::BindGroupEntry {
    wgpu::BindGroupEntry {
      binding: index,
      resource: wgpu::BindingResource::TextureView(&self.inner),
    }
  }
}

/// A structure to encapsulate a [`wgpu::Sampler`].
pub struct TextureSampler {
  inner: wgpu::Sampler,
}

impl TextureSampler {
  /// Creates a new [`TextureSampler`].
  ///
  /// # Arguments
  ///
  /// * `device` - The wgpu device to create the texture sampler.
  /// * `desc` - The [`wgpu::SamplerDescriptor`] to use when creating the
  ///
  /// * `->` A new [`TextureSampler`] created from the given sampler descriptor.
  ///   texture sampler.
  pub fn new(device: &wgpu::Device, desc: &wgpu::SamplerDescriptor) -> Self {
    // Create the texture sampler
    let sampler = device.create_sampler(desc);

    TextureSampler { inner: sampler }
  }
}

/// Implementation of the binding trait to use this in a shader program.
impl Binding for TextureSampler {
  fn layout(&self, index: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
      binding: index,
      visibility: wgpu::ShaderStages::FRAGMENT,
      ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
      count: None,
    }
  }

  fn entry(&self, index: u32) -> wgpu::BindGroupEntry {
    wgpu::BindGroupEntry {
      binding: index,
      resource: wgpu::BindingResource::Sampler(&self.inner),
    }
  }
}
