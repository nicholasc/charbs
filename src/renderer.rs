use std::sync::Arc;

use crate::{
  prelude::{Init, Module, Res},
  window::Window,
};

/// A structure holding wgpu related structures for usage within the library.
///
/// Encapsulates the main wgpu objects within a structure of its own called a
/// [`RenderContext`]. It is used to pass a reference to these objects so that
/// they can be used in different situations without having to pass them around
/// as arguments.
#[derive(Debug)]
pub struct RenderContext {
  adapter: wgpu::Adapter,
  device: wgpu::Device,
  queue: wgpu::Queue,
  surface: wgpu::Surface<'static>,
}

impl RenderContext {
  /// Creates a new [`RenderContext`] structure from an adapter, device, queue
  /// and surface.
  ///
  /// # Arguments
  ///
  /// * `adapter` - The [`wgpu::Adapter`] used for access to the device.
  /// * `device` - The [`wgpu::Device`] used for creating objects on the gpu.
  /// * `queue` - The [`wgpu::Queue`] used to store and execute command buffers.
  /// * `surface` - The [`wgpu::Surface`] the renderer will be drawing to.
  /// * `->` - A new [`RenderContext`] with the provided parameters.
  pub fn new(
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
  ) -> RenderContext {
    Self {
      adapter,
      device,
      queue: queue,
      surface,
    }
  }

  /// Changes the size of the underlying surface that we are rendering to.
  ///
  /// # Arguments
  ///
  /// * `width` - The new width of the render surface.
  /// * `height` - The new height of the render surface.
  pub fn resize(&self, width: u32, height: u32) {
    self.surface.configure(
      &self.device,
      &self
        .surface
        .get_default_config(&self.adapter, width, height)
        .unwrap(),
    );
  }

  /// Returns a read-only reference to the wgpu adapter.
  pub fn adapter(&self) -> &wgpu::Adapter {
    &self.adapter
  }

  /// Returns a read-only reference to the wgpu device.
  pub fn device(&self) -> &wgpu::Device {
    &self.device
  }

  /// Returns a read-only reference to the wgpu command queue.
  pub fn queue(&self) -> &wgpu::Queue {
    &self.queue
  }

  /// Returns a read-only reference to the wgpu surface.
  pub fn surface(&self) -> &wgpu::Surface {
    &self.surface
  }
}

/// A wrapper structure that represents a single frame to be renderered.
///
/// This structure is built to only expose what is needed for a
/// [`crate::app::Runtime`] in order to draw pipelines to the screen.
pub struct RenderFrame {
  encoder: wgpu::CommandEncoder,
  view: wgpu::TextureView,
  texture: wgpu::SurfaceTexture,
}

impl RenderFrame {
  /// Creates a new [`RenderFrame`].
  ///
  /// # Arguments
  ///
  /// * `device` - The [`wgpu::Device`] to use when building the frame.
  /// * `surface` - The [`wgpu::Surface`] to draw to.
  /// * `->` - A new [`RenderFrame`] ready to be drawn to.
  pub fn new(device: &wgpu::Device, surface: &wgpu::Surface<'static>) -> Self {
    // TODO: This fails when texture is 1x1 or less.
    // Might need to pass this as argument and handle texture size
    let texture = surface.get_current_texture().unwrap();

    let view = texture
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    let encoder =
      device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    Self {
      encoder,
      texture,
      view,
    }
  }

  /// Clears the [`RenderFrame`] to a specific color.
  ///
  /// Creates and consumes a [`RenderPass`] with a clear operation that uses
  /// the specified [`wgpu::Color`].
  ///
  /// # Arguments
  ///
  /// * `color` - The [`wgpu::Color`] used to clear the frame.
  pub fn clear(&mut self, color: wgpu::Color) {
    let color_attachments = vec![Some(wgpu::RenderPassColorAttachment {
      view: &self.view,
      resolve_target: None,
      ops: wgpu::Operations {
        store: wgpu::StoreOp::Store,
        load: wgpu::LoadOp::Clear(color),
      },
    })];

    RenderPass::new(&mut self.encoder, color_attachments).finish();
  }

  /// Returns a transparent instance of a [`RenderPass`].
  pub fn create_render_pass(&mut self) -> RenderPass {
    let color_attachments = vec![Some(wgpu::RenderPassColorAttachment {
      view: &self.view,
      resolve_target: None,
      ops: wgpu::Operations {
        store: wgpu::StoreOp::Store,
        load: wgpu::LoadOp::Load,
      },
    })];

    RenderPass::new(&mut self.encoder, color_attachments)
  }

  /// Submits the rendering commands to the [`wgpu::Queue`] and presents the
  /// surface texture on to the screen.
  ///
  /// # Arguments
  ///
  /// * `queue` - The [`wgpu::Queue`] to submit the commands to.
  pub fn finish(self, queue: &wgpu::Queue) {
    queue.submit(std::iter::once(self.encoder.finish()));

    self.texture.present();
  }
}

/// A wrapper structure that encapsulate a [`wgpu::RenderPass`] and offers
/// rendering functionalities to renderable objects.
///
/// # Examples
///
/// ```
/// ...
/// fn render(&self, frame: &mut RenderFrame) {
///   let mut render_pass = frame.create_render_pass();
///   render_pass.render(&self.mesh);
///   render_pass.finish();
/// }
/// ```
pub struct RenderPass<'a> {
  inner: wgpu::RenderPass<'a>,
}

impl<'a> RenderPass<'a> {
  /// Create a new [`RenderPass`] from a [`wgpu::CommandEncoder`] and a
  /// [`wgpu::TextureView`].
  ///
  /// # Arguments
  ///
  /// * `encoder` - The [`wgpu::CommandEncoder`] used to create the RenderPass.
  /// * `color_attachments` - The [`wgpu::RenderPassColorAttachment`] to use.
  /// * -> A new [`RenderPass`] ready to be used for rendering.
  pub fn new(
    encoder: &'a mut wgpu::CommandEncoder,
    color_attachments: Vec<Option<wgpu::RenderPassColorAttachment<'a>>>,
  ) -> Self {
    let inner = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: Some("Nabe::Renderer::RenderCanvas"),
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
      color_attachments: &color_attachments,
    });

    Self { inner }
  }

  /// Renders a structure that implements the [`Renderable`] trait.
  ///
  /// # Arguments
  ///
  /// * `object` - The [`Renderable`] object.
  pub fn render(&mut self, object: &'a dyn Renderable) {
    object.render(&mut self.inner);
  }

  /// Consumes the [`RenderPass`].
  #[inline(always)]
  pub fn finish(self) {}
}

/// A generic trait for objects that can rendered to a [`RenderFrame`].
pub trait Renderable {
  /// Updates the mesh uniform buffers.
  #[inline(always)]
  fn update(&mut self) {}

  /// Renders the object.
  ///
  /// # Arguments
  ///
  /// * `render_pass` - The actual [`wgpu::RenderPass`] to use for rendering.
  fn render<'b>(&'b self, render_pass: &mut wgpu::RenderPass<'b>);
}

pub struct RenderModule;

impl Module for RenderModule {
  fn build(&self, app: &mut crate::prelude::App) {
    app.add_handler(Init, Self::init);
  }
}

impl RenderModule {
  pub fn init(window: Res<Window>) {
    // Create a new instance of a wgpu instance to create our surface from the
    // newly created window and the adapter that will be used to create our
    // rendering context
    let wgpu = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

    // Request an adapter that is compatible with the newly created surface and
    // that ideally is a discrete GPU with high performance
    let adapter = pollster::block_on(async {
      wgpu
        .request_adapter(&wgpu::RequestAdapterOptions {
          power_preference: wgpu::PowerPreference::default(),
          compatible_surface: Some(&window.surface().unwrap()),
          force_fallback_adapter: false,
        })
        .await
        .unwrap()
    });

    // Request a device and a command queue from our adapter
    let (device, queue) = pollster::block_on(async {
      adapter
        .request_device(
          &wgpu::DeviceDescriptor {
            label: Some("Device"),
            memory_hints: wgpu::MemoryHints::default(),
            required_limits: wgpu::Limits::default(),
            required_features: wgpu::Features::empty(),
          },
          None,
        )
        .await
        .unwrap()
    });

    // let ctx = RenderContext::new(adapter, device, queue, surface);
  }
}
