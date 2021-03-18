use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::Section;

use super::{
    accelerator::{Accelerator, AcceleratorInstance},
    camera::Camera,
    color::{self, Color},
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

pub enum Illumination {
    Direct,
    PathTracing,
}

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub accelerator: AcceleratorInstance,
    pub world: World,
    pub primitives: Vec<Primitive>,
    pub camera: Camera,
    pub render_method: RenderMethod,
    pub illumination: Illumination,
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
            illumination: Illumination::Direct,
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
    pub fn illumination<'a>(&'a mut self, illumination: Illumination) -> &'a mut Renderer {
        self.illumination = illumination;
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

        let mut pixels = vec![Color::default(); height * width];
        (0..height * width)
            .into_par_iter()
            .map(|pixel| {
                let x = (left + pixel % width) as f32;
                let y = (top + pixel / width) as f32;
                let ray = camera.get_ray(x, y);

                self.trace_ray(&ray, 0)
            })
            .collect_into_vec(&mut pixels);
        return pixels;
    }
    fn render_tiles(&mut self, section: &Section) -> Vec<Color> {
        let height = section.height;
        let width = section.width;
        let camera = &self.camera;
        let tile_size = 16;
        let sections_v = height / tile_size;
        let sections_h = width / tile_size;
        let pixels_per_tile = tile_size * tile_size;

        let tiles = (0..sections_v * sections_h)
            .into_par_iter()
            .map(|idx| {
                let x = section.x + (idx % sections_h) * tile_size;
                let y = section.y + (idx / sections_h) * tile_size;
                let pixels: Vec<Color> = Vec::with_capacity(pixels_per_tile);
                (x, y, pixels)
            })
            .map(|(x, y, mut pixels)| {
                for yy in 0..tile_size {
                    for xx in 0..tile_size {
                        let ray = camera.get_ray((x + xx) as f32, (y + yy) as f32);
                        pixels.push(self.trace_ray(&ray, 0));
                    }
                }
                pixels
            })
            .collect::<Vec<Vec<Color>>>();

        let mut pixels: Vec<Color> = vec![Color::default(); width * height];
        for (section, colors) in tiles.into_iter().enumerate() {
            let start_x = (section % sections_h) * tile_size;
            let start_y = (section / sections_h) * tile_size;
            for (idx, color) in colors.into_iter().enumerate() {
                let x = idx % tile_size;
                let y = idx / tile_size;
                pixels[(start_y + y) * width + start_x + x] = color;
            }
        }
        return pixels;
    }

    fn trace_ray(&self, ray: &Ray, depth: u8) -> Color {
        let accelerator = &self.accelerator;
        if depth > 10 {
            return color::BLACK;
        }

        match accelerator.trace(&ray) {
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
                    None => color::BLACK,
                }
            }
            None => color::BLACK,
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
