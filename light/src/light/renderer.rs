use super::{
    accelerator::{Accelerator, AcceleratorInstance, AcceleratorStats},
    camera::Camera,
    color::Color,
    path_tracing,
    primitive::Primitive,
    section::{Section, SectionIterator},
    tile::{Tile, TileIterator},
    whitted,
    world::World,
};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

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
    pub width: usize,
    pub height: usize,
    pub accelerator: AcceleratorInstance,
    pub world: World,
    pub primitives: Vec<Primitive>,
    pub camera: Camera,
    pub render_method: RenderMethod,
    pub algorithm: Algorithm,
    pub stats: Option<Stats>,
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
        }
    }
    pub fn algorithm(&mut self, algorithm: Algorithm) -> &mut Renderer {
        self.algorithm = algorithm;
        self
    }
    pub fn width(&mut self, width: usize) -> &mut Renderer {
        self.width = width;
        self
    }
    pub fn height(&mut self, height: usize) -> &mut Renderer {
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
    pub fn threads(&mut self, count: usize) -> &mut Renderer {
        if count > 0 {
            rayon::ThreadPoolBuilder::new()
                .num_threads(count)
                .build_global()
                .unwrap();
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
        let it = SectionIterator::new(left, top, width, height);

        let trace = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };

        let mut pixels = vec![Color::default(); height * width];
        it.into_par_iter()
            .map_init(rand::thread_rng, |rng, pixel| trace(self, rng, pixel))
            .collect_into_vec(&mut pixels);
        pixels
    }
    fn render_tiles(&mut self, section: &Section) -> Vec<Color> {
        let height = section.height;
        let width = section.width;
        let tile_size = 16;
        let sections_h = width / tile_size;
        let it = TileIterator::new(section.x, section.y, section.width, section.height);

        let trace = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };

        let tiles = it
            .into_par_iter()
            .map_init(rand::thread_rng, |rnd, Tile { x, y, size }| {
                let mut pixels = vec![];
                for pixel in SectionIterator::new(x, y, size, size) {
                    pixels.push(trace(self, rnd, pixel));
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
            .map_init(rand::thread_rng, |rng, row| {
                let y = top + row;

                Iterator::map(SectionIterator::new(0, y, width, height), |pixel| {
                    trace(self, rng, pixel)
                })
                .collect::<Vec<Color>>()
            })
            .flatten()
            .collect::<Vec<Color>>()
    }
}
