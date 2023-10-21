use super::{
    accelerators::{Accelerator, AcceleratorInstance, AcceleratorStats},
    algorithms::{path_tracing, whitted, Algorithm},
    camera::Camera,
    color::Color,
    float::Float,
    primitives::Primitive,
    render_method::{RenderMethod, TraceFn},
    section::Section,
    world::World,
};

#[derive(Clone, Debug, Default)]
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
    pub fn builder() -> RendererBuilder {
        RendererBuilder {
            width: 0,
            height: 0,
            accelerator: Accelerator::BruteForce,
            camera: Camera::default(),
            world: World::default(),
            render_method: RenderMethod::Pixels,
            algorithm: Algorithm::Whitted,
            stats: None,
            threads: None,
            samples: 1,
        }
    }

    pub fn render(&mut self, section: &Section) -> Vec<Color> {
        let trace: TraceFn = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };
        // let render: RenderFn = match (&self.threads, &self.stats, &self.render_method) {
        //     (Some(1), _, _) | (_, Some(_), _) => render_pixels_st,
        //     (_, _, RenderMethod::Pixels) => render_pixels,
        //     (_, _, RenderMethod::Tiles) => render_tiles,
        //     (_, _, RenderMethod::Scanlines) => render_scanlines,
        // };

        let render = self.render_method.get();
        render(self, section, trace)
    }
}

#[derive(Clone)]
pub struct RendererBuilder {
    pub width: u32,
    pub height: u32,
    pub accelerator: Accelerator,
    pub world: World,
    pub camera: Camera,
    pub render_method: RenderMethod,
    pub algorithm: Algorithm,
    pub stats: Option<Stats>,
    pub threads: Option<u32>,
    pub samples: u32,
}

impl RendererBuilder {
    pub fn algorithm(&mut self, algorithm: Algorithm) -> &mut RendererBuilder {
        self.algorithm = algorithm;
        self
    }
    pub fn width(&mut self, width: u32) -> &mut RendererBuilder {
        self.width = width;
        self
    }
    pub fn height(&mut self, height: u32) -> &mut RendererBuilder {
        self.height = height;
        self
    }
    pub fn camera(&mut self, camera: Camera) -> &mut RendererBuilder {
        self.camera = camera;
        self
    }
    pub fn world(&mut self, world: World) -> &mut RendererBuilder {
        self.world = world;
        self
    }
    pub fn render_method(&mut self, render_method: RenderMethod) -> &mut RendererBuilder {
        self.render_method = render_method;
        self
    }
    pub fn illumination(&mut self, algorithm: Algorithm) -> &mut RendererBuilder {
        self.algorithm = algorithm;
        self
    }
    pub fn accelerator(&mut self, accelerator: Accelerator) -> &mut RendererBuilder {
        self.accelerator = accelerator;
        self
    }
    pub fn threads(&mut self, count: u32) -> &mut RendererBuilder {
        self.threads = if count > 0 { Some(count) } else { None };
        rayon::ThreadPoolBuilder::new()
            .num_threads(count as usize)
            .build_global()
            .unwrap();
        self
    }
    pub fn samples(&mut self, samples: u32) -> &mut RendererBuilder {
        self.samples = samples;
        self
    }
    pub fn use_stats(&mut self) -> &mut RendererBuilder {
        self.stats = Some(Stats::default());
        self
    }
    pub fn build(&mut self) -> Renderer {
        let RendererBuilder {
            width,
            height,
            accelerator,
            world,
            camera,
            render_method,
            algorithm,
            stats,
            threads,
            samples,
        } = self.to_owned();

        let mut camera = camera;
        camera.init(self.width as Float, self.height as Float);

        let primitives = world.primitives();

        let accelerator = match accelerator {
            Accelerator::BruteForce => AcceleratorInstance::new_brute_force(&primitives),
            Accelerator::BoundingVolumeHierarchy => {
                AcceleratorInstance::new_bounding_volume_hierarchy(&primitives)
            }
        };

        Renderer {
            width,
            height,
            accelerator,
            world,
            primitives,
            camera,
            render_method,
            algorithm,
            stats,
            threads,
            samples,
        }
    }
}
