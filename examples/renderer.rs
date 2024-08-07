use charbs::{materials::DefaultMaterials, prelude::*, renderer::RendererModule};

fn main() {
  App::default()
    .add_module(WindowModule)
    .add_module(RenderModule)
    .add_module(RendererModule)
    .add_module(DefaultMaterials)
    .run();
}
