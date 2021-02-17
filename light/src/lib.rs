pub mod demos;
mod light;
pub use light::camera::Camera;
pub use light::color::Color;
pub use light::light::Light;
pub use light::material::Material;
pub use light::renderer::Renderer;
pub use light::solid::Solid;
pub use light::transform::Transform;
pub use light::vector::Vector;
pub use light::world::World;

/*
World::new()
.camera()
.lights()
.objects()
.finish()

Renderer::new()
.width()
.height()
.world()
.acc_structure()
.finish()
*/
