use charbs::{
  app::Commands,
  materials::{ColorMaterial, DefaultMaterials},
  math::Rectangle,
  mesh::{Mesh, MeshInstance},
  prelude::*,
  renderer::RendererModule,
  resources::Resources,
  transform::Transform,
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

pub fn init(
  ctx: Res<RenderContext>,
  mut commands: ResMut<Commands>,
  mut meshes: ResMut<Resources<Mesh>>,
  mut materials: ResMut<Resources<ColorMaterial>>,
) {
  commands.spawn(MeshInstance {
    mesh: meshes.add(Rectangle::new(0.5, 0.5)),
    material: materials.add(ColorMaterial::new(ctx.device(), 0.0, 0.0, 1.0)),
    transform: Transform::default(),
  });
}
