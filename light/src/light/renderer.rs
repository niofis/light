use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::{
    accelerator::Accelerator, color::Color, direct_illumination, material::Material,
    primitive::Primitive, ray::Ray, transform::Transform, vector::Vector, world::World,
};
// use rayon::prelude::*;
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub accelerator: Accelerator,
    pub world: World,
    pub primitives: Vec<Primitive>,
}

impl Renderer {
    pub fn build() -> Renderer {
        Renderer {
            width: 0,
            height: 0,
            accelerator: Accelerator::None,
            world: World::default(),
            primitives: Vec::new(),
        }
    }
    pub fn width(mut self, width: usize) -> Renderer {
        self.width = width;
        self
    }
    pub fn height(mut self, height: usize) -> Renderer {
        self.height = height;
        self
    }
    pub fn accelerator(mut self, accelerator: Accelerator) -> Renderer {
        self.accelerator = accelerator;
        self
    }
    pub fn world(mut self, world: World) -> Renderer {
        self.world = world;
        self
    }
    pub fn finish(mut self) -> Renderer {
        self.primitives = self.world.primitives();
        self
    }

    pub fn render(&mut self) -> Vec<u8> {
        let height = self.height;
        let width = self.width;
        let camera = &self.world.camera;

        let pixels = (0..height * width)
            .into_par_iter()
            .map(|pixel| {
                let x = (pixel % width) as f32;
                let y = (pixel / width) as f32;
                let ray = camera.get_ray(x, y);

                self.trace_ray(&self.accelerator, &self.world, ray, 0)
            })
            .collect::<Vec<Color>>();

        let mut buffer: Vec<u8> = vec![0; (4 * width * height) as usize];
        let mut offset = 0;
        for pixel in pixels {
            let Color(r, g, b) = pixel;
            buffer[offset] = if b > 1.0 { 255 } else { (b * 255.99) as u8 };
            buffer[offset + 1] = if g > 1.0 { 255 } else { (g * 255.99) as u8 };
            buffer[offset + 2] = if r > 1.0 { 255 } else { (r * 255.99) as u8 };
            offset = offset + 4;
        }
        buffer
    }

    pub fn rotate_camera(&mut self, rads: f32) {
        let rotation = Transform::rotate(0.0, rads, 0.0);
        let mut camera = &mut self.world.camera;
        camera.left_top = rotation.apply(&camera.left_top);
        camera.delta_down = rotation.apply(&camera.delta_down);
        camera.delta_right = rotation.apply(&camera.delta_right);
        camera.eye = rotation.apply(&camera.eye);
    }

    // pub fn rotate_light(&mut self, rads: f32) {
    //     let rotation = Transform::rotate(0.0, rads, 0.0);
    //     let point_lights = &mut self.world.lights;
    //     point_lights[0] = rotation.apply(&point_lights[0]);
    // }

    fn trace_ray(&self, tracer: &Accelerator, world: &World, ray: Ray, depth: u8) -> Color {
        if depth > 10 {
            return Color(0.0, 0.0, 0.0);
        }

        match tracer.trace(&ray) {
            Some(prm_idxs) => {
                let closest = self.find_closest_primitive(&ray, &prm_idxs);
                match closest {
                    Some((primitive, distance)) => {
                        let point = ray.point(distance);
                        let prm_material = match primitive {
                            Primitive::Sphere { material, .. } => material,
                            Primitive::Triangle { material, .. } => material,
                        };

                        match prm_material {
                            Material::Simple(_) => {
                                self.calculate_shading(world, &primitive, &point)
                            }
                            Material::Reflective(_, idx) => {
                                let normal = primitive.normal(&point);
                                let ri = ray.1.unit();
                                let dot = ri.dot(&normal) * 2.0;
                                let new_dir = &ri - &(&normal * dot);
                                let reflected_ray = Ray::new(&point, &new_dir.unit());
                                (self.calculate_shading(world, &primitive, &point) * (1.0 - idx))
                                    + self.trace_ray(tracer, world, reflected_ray, depth + 1) * *idx
                            }
                        }
                    }
                    None => Color(0.0, 0.0, 0.0),
                }
            }
            None => Color(0.0, 0.0, 0.0),
        }
    }

    fn calculate_shading(&self, world: &World, prm: &Primitive, point: &Vector) -> Color {
        let normal = prm.normal(point);
        let direct_lighting = direct_illumination::calculate(self, &point, &normal);

        let prm_material = match prm {
            Primitive::Sphere { material, .. } => material,
            Primitive::Triangle { material, .. } => material,
        };

        let prm_color = match prm_material {
            Material::Simple(color) => color,
            Material::Reflective(color, _) => color,
        };

        Color(
            prm_color.0 * direct_lighting.0,
            prm_color.1 * direct_lighting.1,
            prm_color.2 * direct_lighting.2,
        )
    }

    fn find_closest_primitive<'a>(
        &'a self,
        ray: &Ray,
        prm_indexes: &[usize],
    ) -> Option<(&'a Primitive, f32)> {
        let primitives = &self.primitives;
        prm_indexes
            .iter()
            .filter_map(|idx| {
                primitives[*idx]
                    .intersect(ray)
                    .map(|dist| (&primitives[*idx], dist))
            })
            .fold(None, |closest, (pr, dist)| match closest {
                None => Some((pr, dist)),
                Some(res) if dist < res.1 => Some((pr, dist)),
                _ => closest,
            })
    }
}
