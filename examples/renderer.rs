use charbs::{prelude::*, renderer::RendererModule};

fn main() {
  App::default()
    .add_module(WindowModule)
    .add_module(RenderModule)
    .add_module(RendererModule)
    .run();
}
