use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::Section;

use super::{
    accelerator::{Accelerator, AcceleratorInstance},
    camera::Camera,
    color::Color,
    direct_illumination,
    material::Material,
    primitive::Primitive,
    ray::Ray,
    vector::Vector,
    world::World,
};

pub enum RenderMethod {
    Pixels,
    Tiles,
}

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub accelerator: AcceleratorInstance,
    pub world: World,
    pub primitives: Vec<Primitive>,
    pub camera: Camera,
    pub render_method: RenderMethod,
}

impl Renderer {
    pub fn build() -> Renderer {
        Renderer {
            width: 0,
            height: 0,
            accelerator: AcceleratorInstance::None,
            camera: Camera::default(),
            world: World::default(),
            primitives: Vec::new(),
            render_method: RenderMethod::Pixels,
        }
    }
    pub fn width<'a>(&'a mut self, width: usize) -> &'a mut Renderer {
        self.width = width;
        self
    }
    pub fn height<'a>(&'a mut self, height: usize) -> &'a mut Renderer {
        self.height = height;
        self
    }
    pub fn camera<'a>(&'a mut self, camera: Camera) -> &'a mut Renderer {
        self.camera = camera;
        self.camera.init(self.width as f32, self.height as f32);
        self
    }
    pub fn world<'a>(&'a mut self, world: World) -> &'a mut Renderer {
        self.world = world;
        self.primitives = self.world.primitives();
        self
    }
    pub fn render_method<'a>(&'a mut self, render_method: RenderMethod) -> &'a mut Renderer {
        self.render_method = render_method;
        self
    }
    pub fn accelerator<'a>(&'a mut self, accelerator: Accelerator) -> &'a mut Renderer {
        self.accelerator = match accelerator {
            Accelerator::BruteForce => AcceleratorInstance::new_brute_force(&self.primitives),
            Accelerator::BoundingVolumeHierarchy => {
                AcceleratorInstance::new_bounding_volume_hierarchy(&self.primitives)
            }
        };
        self
    }
    pub fn threads<'a>(&'a mut self, count: usize) -> &'a mut Renderer {
        if count > 0 {
            rayon::ThreadPoolBuilder::new()
                .num_threads(count)
                .build_global()
                .unwrap();
        }
        self
    }
    pub fn finish(&mut self) -> &Renderer {
        self
    }
    pub fn render(&mut self, section: &Section) -> Vec<Color> {
        match self.render_method {
            RenderMethod::Pixels => self.render_pixels(section),
            RenderMethod::Tiles => self.render_tiles(section),
        }
    }
    fn render_pixels(&mut self, section: &Section) -> Vec<Color> {
        let height = section.height;
        let width = section.width;
        let left = section.x;
        let top = section.y;
        let camera = &self.camera;

        let pixels = (0..height * width)
            .into_par_iter()
            .map(|pixel| {
                let x = (left + pixel % width) as f32;
                let y = (top + pixel / width) as f32;
                let ray = camera.get_ray(x, y);

                self.trace_ray(&ray, 0)
            })
            .collect::<Vec<Color>>();
        return pixels;
    }
    fn render_tiles(&mut self, section: &Section) -> Vec<Color> {
        let height = section.height;
        let width = section.width;
        let camera = &self.camera;
        let section_size = 16;
        let sections_v = height / section_size;
        let sections_h = width / section_size;
        let pixels_per_section = section_size * section_size;
        let results = (0..sections_v * sections_h)
            .into_par_iter()
            .map(|idx| {
                let left = section.x + (idx % sections_h) * section_size;
                let top = section.y + (idx / sections_h) * section_size;
                let right = left + section_size;
                let bottom = top + section_size;
                let mut pixels = Vec::with_capacity(pixels_per_section);
                for y in top..bottom {
                    for x in left..right {
                        let ray = camera.get_ray(x as f32, y as f32);
                        let pixel = self.trace_ray(&ray, 0);
                        pixels.push(pixel);
                    }
                }
                pixels
            })
            .collect::<Vec<Vec<Color>>>();
        let mut pixels: Vec<Color> = vec![Color::default(); width * height];
        for (section, colors) in results.into_iter().enumerate() {
            let start_x = (section % sections_h) * section_size;
            let start_y = (section / sections_h) * section_size;
            for (idx, color) in colors.into_iter().enumerate() {
                let x = idx % section_size;
                let y = idx / section_size;
                pixels[(start_y + y) * width + start_x + x] = color;
            }
        }
        return pixels;
    }

    // pub fn rotate_camera(&mut self, rads: f32) {
    //     let rotation = Transform::rotate(0.0, rads, 0.0);
    //     let mut camera = &mut self.camera;
    //     camera.left_top = rotation.apply(&camera.left_top);
    //     camera.delta_down = rotation.apply(&camera.delta_down);
    //     camera.delta_right = rotation.apply(&camera.delta_right);
    //     camera.eye = rotation.apply(&camera.eye);
    // }

    // pub fn rotate_light(&mut self, rads: f32) {
    //     let rotation = Transform::rotate(0.0, rads, 0.0);
    //     let point_lights = &mut self.world.lights;
    //     point_lights[0] = rotation.apply(&point_lights[0]);
    // }

    fn trace_ray(&self, ray: &Ray, depth: u8) -> Color {
        let tracer = &self.accelerator;
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
                            Material::Simple(_) => self.calculate_shading(&primitive, &point),
                            Material::Reflective(_, idx) => {
                                let normal = primitive.normal(&point);
                                let ri = ray.1.unit();
                                let dot = ri.dot(&normal) * 2.0;
                                let new_dir = &ri - &(&normal * dot);
                                let reflected_ray = Ray::new(&point, &new_dir.unit());
                                (self.calculate_shading(&primitive, &point) * (1.0 - idx))
                                    + self.trace_ray(&reflected_ray, depth + 1) * *idx
                            }
                        }
                    }
                    None => Color(0.0, 0.0, 0.0),
                }
            }
            None => Color(0.0, 0.0, 0.0),
        }
    }

    fn calculate_shading(&self, prm: &Primitive, point: &Vector) -> Color {
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
