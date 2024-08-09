use charbs::{
  materials::{ColorMaterial, DefaultMaterials, Materials},
  prelude::*,
  renderer::RendererModule,
};

fn main() {
  App::default()
    .add_module(WindowModule)
    .add_module(RenderModule)
    .add_module(RendererModule)
    .add_module(DefaultMaterials)
    .add_handler(Init, init)
    .run();
}

pub fn init(ctx: Res<RenderContext>, mut materials: ResMut<Materials>) {
  materials.add(Box::new(ColorMaterial::new(ctx.device(), 1.0, 0.0, 0.0)));
}
