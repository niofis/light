use super::{
    accelerator::{Accelerator, AcceleratorInstance, AcceleratorStats},
    camera::Camera,
    color::Color,
    path_tracing,
    primitive::Primitive,
    section::Section,
    whitted,
    world::World,
};
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub enum RenderMethod {
    Pixels,
    Tiles,
    Scanlines,
}

pub enum Algorithm {
    Whitted,
    PathTracing,
}

#[derive(Debug, Default)]
pub struct Stats {
    primitives: usize,
    accelerator: Option<AcceleratorStats>,
    // bvh_nodes: usize,
    // bvh_height: usize,
    // bvh_leaves: usize,
    // bvh_ppn: usize,
}

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub accelerator: AcceleratorInstance,
    pub world: World,
    pub primitives: Vec<Primitive>,
    pub camera: Camera,
    pub render_method: RenderMethod,
    pub algorithm: Algorithm,
    pub stats: Option<Stats>,
    pub threads: Option<u32>,
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
            algorithm: Algorithm::Whitted,
            stats: None,
            threads: None,
        }
    }
    pub fn algorithm(&mut self, algorithm: Algorithm) -> &mut Renderer {
        self.algorithm = algorithm;
        self
    }
    pub fn width(&mut self, width: u32) -> &mut Renderer {
        self.width = width;
        self
    }
    pub fn height(&mut self, height: u32) -> &mut Renderer {
        self.height = height;
        self
    }
    pub fn camera(&mut self, camera: Camera) -> &mut Renderer {
        self.camera = camera;
        self.camera.init(self.width as f32, self.height as f32);
        self
    }
    pub fn world(&mut self, world: World) -> &mut Renderer {
        self.world = world;
        self.primitives = self.world.primitives();
        if let Some(mut stats) = self.stats.as_mut() {
            stats.primitives = self.primitives.len();
        }
        self
    }
    pub fn render_method(&mut self, render_method: RenderMethod) -> &mut Renderer {
        self.render_method = render_method;
        self
    }
    pub fn illumination(&mut self, algorithm: Algorithm) -> &mut Renderer {
        self.algorithm = algorithm;
        self
    }
    pub fn accelerator(&mut self, accelerator: Accelerator) -> &mut Renderer {
        self.accelerator = match accelerator {
            Accelerator::BruteForce => AcceleratorInstance::new_brute_force(&self.primitives),
            Accelerator::BoundingVolumeHierarchy => {
                let acc = AcceleratorInstance::new_bounding_volume_hierarchy(&self.primitives);
                if let Some(mut stats) = self.stats.as_mut() {
                    stats.accelerator = Some(acc.stats());
                }
                acc
            }
        };
        self
    }
    pub fn threads(&mut self, count: u32) -> &mut Renderer {
        if count > 0 {
            rayon::ThreadPoolBuilder::new()
                .num_threads(count as usize)
                .build_global()
                .unwrap();
            self.threads = Some(count);
        }
        self
    }
    pub fn use_stats(&mut self) -> &mut Renderer {
        self.stats = Some(Stats::default());
        self
    }
    pub fn finish(&mut self) -> &Renderer {
        self
    }
    pub fn render(&mut self, section: &Section) -> Vec<Color> {
        if let Some(1) = self.threads {
            return self.render_pixels_single_thread(section);
        }
        if self.stats.is_some() {
            return self.render_pixels_single_thread(section);
        }
        match self.render_method {
            RenderMethod::Pixels => self.render_pixels(section),
            RenderMethod::Tiles => self.render_tiles(section),
            RenderMethod::Scanlines => self.render_scanlines(section),
        }
    }
    fn render_pixels(&mut self, section: &Section) -> Vec<Color> {
        let height = section.height;
        let width = section.width;
        let left = section.x;
        let top = section.y;
        let trace = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };

        (0..width * height)
            .into_par_iter()
            .map(|idx| (left + (idx % width), top + (idx / width)))
            .map_init(Xoshiro256PlusPlus::from_entropy, |rng, pixel| {
                trace(self, rng, pixel)
            })
            .collect()
    }
    fn render_tiles(&mut self, section: &Section) -> Vec<Color> {
        let Section {
            x: left,
            y: top,
            height,
            width,
        } = section;

        let trace = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };

        let tile_size = 16;
        let sections_v = height / tile_size;
        let sections_h = width / tile_size;

        let tiles: Vec<(u32, u32)> = (0..sections_v * sections_h)
            .map(|idx| {
                let x = left + (idx % sections_h) * tile_size;
                let y = top + (idx / sections_h) * tile_size;
                (x, y)
            })
            .collect();

        let tiles = tiles
            .into_par_iter()
            .map_init(Xoshiro256PlusPlus::from_entropy, |rnd, (x, y)| {
                (0..tile_size * tile_size)
                    .map(|idx| (x + (idx % tile_size), y + (idx / tile_size)))
                    .map(|pixel| trace(self, rnd, pixel))
                    .collect()
            })
            .collect::<Vec<Vec<Color>>>();

        let mut pixels: Vec<Color> = vec![Color::default(); (width * height) as usize];
        for (section, colors) in tiles.into_iter().enumerate() {
            let start_x = (section as u32 % sections_h) * tile_size;
            let start_y = (section as u32 / sections_h) * tile_size;
            for (idx, color) in colors.into_iter().enumerate() {
                let x = idx as u32 % tile_size;
                let y = idx as u32 / tile_size;
                pixels[((start_y + y) * width + start_x + x) as usize] = color;
            }
        }

        pixels
    }
    fn render_scanlines(&mut self, section: &Section) -> Vec<Color> {
        let height = section.height;
        let width = section.width;
        let top = section.y;

        let trace = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };

        (0..height)
            .into_par_iter()
            .map_init(Xoshiro256PlusPlus::from_entropy, |rng, row| {
                let y = top + row;

                (0..width)
                    .map(|idx| (idx, y))
                    .map(|pixel| trace(self, rng, pixel))
                    .collect::<Vec<Color>>()
            })
            .flatten()
            .collect::<Vec<Color>>()
    }

    fn render_pixels_single_thread(&mut self, section: &Section) -> Vec<Color> {
        let height = section.height;
        let width = section.width;
        let left = section.x;
        let top = section.y;
        let trace = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };
        // let mut rng = rand::thread_rng();

        let mut rng = Xoshiro256PlusPlus::from_entropy();

        (0..width * height)
            .map(|idx| (left + (idx % width), top + (idx / width)))
            .map(|pixel| trace(self, &mut rng, pixel))
            .collect()
    }
}
