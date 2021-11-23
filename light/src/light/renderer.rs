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
    bvh_nodes: usize,
    bvh_height: usize,
    bvh_leaves: usize,
    bvh_ppn: usize,
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
        if let Some(mut stats) = self.stats.as_mut() {
            stats.primitives = self.primitives.len();
        }
        self
    }
    pub fn render_method<'a>(&'a mut self, render_method: RenderMethod) -> &'a mut Renderer {
        self.render_method = render_method;
        self
    }
    pub fn illumination<'a>(&'a mut self, algorithm: Algorithm) -> &'a mut Renderer {
        self.algorithm = algorithm;
        self
    }
    pub fn accelerator<'a>(&'a mut self, accelerator: Accelerator) -> &'a mut Renderer {
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
    pub fn threads<'a>(&'a mut self, count: usize) -> &'a mut Renderer {
        if count > 0 {
            rayon::ThreadPoolBuilder::new()
                .num_threads(count)
                .build_global()
                .unwrap();
        }
        self
    }
    pub fn use_stats<'a>(&'a mut self) -> &'a mut Renderer {
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
        let camera = &self.camera;
        let it = SectionIterator::new(left, top, width, height);

        let trace = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };

        let mut pixels = vec![Color::default(); height * width];
        //(0..height * width)
        it.into_par_iter()
            .map_init(rand::thread_rng, |rng, pixel| {
                let ray = camera.get_ray(pixel.x, pixel.y);
                trace(self, rng, &ray, 0)
            })
            .collect_into_vec(&mut pixels);
        pixels
    }
    fn render_tiles(&mut self, section: &Section) -> Vec<Color> {
        let height = section.height;
        let width = section.width;
        let camera = &self.camera;
        let tile_size = 16;
        let sections_v = height / tile_size;
        let sections_h = width / tile_size;
        let pixels_per_tile = tile_size * tile_size;
        let it = TileIterator::new(section.x, section.y, section.width, section.height);

        let trace = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };

        let tiles = it
            .into_par_iter()
            .map_init(rand::thread_rng, |rnd, Tile { x, y, size }| {
                let mut pixels = vec![];
                for yy in 0..size {
                    for xx in 0..size {
                        let ray = camera.get_ray((x + xx) as f32, (y + yy) as f32);
                        pixels.push(trace(&self, rnd, &ray, 0));
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
        pixels
    }
    fn render_scanlines(&mut self, section: &Section) -> Vec<Color> {
        let height = section.height;
        let width = section.width;
        let left = section.x;
        let top = section.y;
        let camera = &self.camera;

        let trace = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };

        //let mut pixels = vec![Color::default(); height * width];
        (0..height)
            .into_par_iter()
            .map_init(rand::thread_rng, |rng, row| {
                let y = top + row;

                (0..width)
                    .into_iter()
                    .map(|col| camera.get_ray((left + col) as f32, y as f32))
                    .map(|ray| trace(self, rng, &ray, 0))
                    .collect::<Vec<Color>>()
            })
            .flatten()
            .collect::<Vec<Color>>()
    }
}
