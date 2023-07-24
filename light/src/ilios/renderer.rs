use super::{
    accelerators::{Accelerator, AcceleratorInstance, AcceleratorStats},
    algorithms::{path_tracing, whitted, Algorithm},
    camera::Camera,
    color::Color,
    float::Float,
    parsers::json::parse_scene,
    primitives::Primitive,
    rng::{Rng, XorRng},
    section::Section,
    world::World,
};
#[cfg(not(target_arch = "wasm32"))]
use rayon::iter::{IntoParallelIterator, ParallelIterator};

type TraceFn = fn(&Renderer, &mut dyn Rng, (u32, u32)) -> Color;
type RenderFn = fn(&mut Renderer, &Section, TraceFn) -> Vec<Color>;

#[derive(Debug)]
pub enum RenderMethod {
    Pixels,
    Tiles,
    Scanlines,
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

#[derive(Debug)]
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
    pub samples: u32,
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
            samples: 1,
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
        self.camera.init(self.width as Float, self.height as Float);
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
    pub fn from_json(&mut self, json: &str) -> &mut Renderer {
        let (camera, world) = parse_scene(json);
        self.camera(camera);
        self.world(world);
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
        #[cfg(not(target_arch = "wasm32"))]
        if count > 0 {
            rayon::ThreadPoolBuilder::new()
                .num_threads(count as usize)
                .build_global()
                .unwrap();
            self.threads = Some(count);
        }
        self
    }
    pub fn samples(&mut self, samples: u32) -> &mut Renderer {
        self.samples = samples;
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
        let trace: TraceFn = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };
        let render: RenderFn = match (&self.threads, &self.stats, &self.render_method) {
            (Some(1), _, _) | (_, Some(_), _) => render_pixels_st,
            (_, _, RenderMethod::Pixels) => render_pixels,
            (_, _, RenderMethod::Tiles) => render_tiles,
            (_, _, RenderMethod::Scanlines) => render_scanlines,
        };
        render(self, section, trace)
    }
}

fn render_pixels(renderer: &mut Renderer, section: &Section, trace: TraceFn) -> Vec<Color> {
    #[cfg(not(target_arch = "wasm32"))]
    return render_pixels_mt(renderer, section, trace);
    #[cfg(target_arch = "wasm32")]
    return render_pixels_st(renderer, section, trace);
}

#[cfg(not(target_arch = "wasm32"))]
fn render_pixels_mt(renderer: &mut Renderer, section: &Section, trace: TraceFn) -> Vec<Color> {
    let Section {
        left,
        top,
        height,
        width,
    } = section;

    (0..width * height)
        .into_par_iter()
        .map(|idx| (left + (idx % width), top + (idx / width)))
        .map_init(|| XorRng::new(), |rng, pixel| trace(renderer, rng, pixel))
        .collect()
}

fn render_pixels_st(renderer: &mut Renderer, section: &Section, trace: TraceFn) -> Vec<Color> {
    let Section {
        height,
        width,
        left,
        top,
    } = *section;
    let mut rng = XorRng::new();

    (top..height)
        .flat_map(|y| (left..width).map(move |x| (x, y)))
        .map(|pixel| trace(renderer, &mut rng, pixel))
        .collect()
}

fn render_tiles(renderer: &mut Renderer, section: &Section, trace: TraceFn) -> Vec<Color> {
    #[cfg(not(target_arch = "wasm32"))]
    return render_tiles_mt(renderer, section, trace);
    #[cfg(target_arch = "wasm32")]
    return render_pixels_st(renderer, section, trace);
}

#[cfg(not(target_arch = "wasm32"))]
fn render_tiles_mt(renderer: &mut Renderer, section: &Section, trace: TraceFn) -> Vec<Color> {
    let Section {
        left,
        top,
        height,
        width,
    } = section;
    let tile_size = 4;
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
        .map_init(
            || XorRng::new(),
            |rnd, (x, y)| {
                (0..tile_size * tile_size)
                    .map(|idx| (x + (idx % tile_size), y + (idx / tile_size)))
                    .map(|pixel| trace(renderer, rnd, pixel))
                    .collect()
            },
        )
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

fn render_scanlines(renderer: &mut Renderer, section: &Section, trace: TraceFn) -> Vec<Color> {
    #[cfg(not(target_arch = "wasm32"))]
    return render_scanlines_mt(renderer, section, trace);
    #[cfg(target_arch = "wasm32")]
    return render_pixels_st(renderer, section, trace);
}

#[cfg(not(target_arch = "wasm32"))]
fn render_scanlines_mt(renderer: &mut Renderer, section: &Section, trace: TraceFn) -> Vec<Color> {
    let Section {
        height, width, top, ..
    } = section;

    (0..*height)
        .into_par_iter()
        .map_init(
            || XorRng::new(),
            |rng, row| {
                let y = top + row;

                (0..*width)
                    .map(|idx| (idx, y))
                    .map(|pixel| trace(renderer, rng, pixel))
                    .collect::<Vec<Color>>()
            },
        )
        .flatten()
        .collect::<Vec<Color>>()
}
