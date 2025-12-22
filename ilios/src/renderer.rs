use ilios_types::{camera::Camera, color::Color, float::Float, section::Section, world::World};

use super::{
    accelerators::{Accelerator, AcceleratorInstance, BvhBuildMethod},
    algorithms::{Algorithm, path_tracing, whitted},
    render_method::{RenderMethod, TraceFn},
};

#[derive(Debug)]
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub accelerator: AcceleratorInstance,
    pub world: World,
    pub camera: Camera,
    pub render_method: RenderMethod,
    pub algorithm: Algorithm,
    pub threads: Option<u32>,
    pub samples: u32,
    accelerator_type: Accelerator,
    bvh_build_method: BvhBuildMethod,
}

impl Renderer {
    pub fn new(builder: &RendererBuilder) -> Renderer {
        let RendererBuilder {
            width,
            height,
            accelerator,
            world,
            camera,
            render_method,
            algorithm,
            threads,
            samples,
            bvh_build_method,
        } = builder.clone();

        let mut camera = camera;
        camera.init(width as Float, height as Float);

        let primitives = world.primitives();

        let accelerator_instance = match accelerator {
            Accelerator::BruteForce => AcceleratorInstance::new_brute_force(primitives),
            Accelerator::BoundingVolumeHierarchy => {
                AcceleratorInstance::new_bounding_volume_hierarchy(bvh_build_method, primitives)
            }
        };

        if let Some(trds) = threads {
            _ = rayon::ThreadPoolBuilder::new()
                .num_threads(trds as usize)
                .build_global();
        }

        eprintln!("Threads: {}", rayon::current_num_threads());

        Renderer {
            width,
            height,
            accelerator: accelerator_instance,
            world,
            camera,
            render_method,
            algorithm,
            threads,
            samples,
            accelerator_type: accelerator,
            bvh_build_method,
        }
    }

    pub fn builder() -> RendererBuilder {
        RendererBuilder {
            width: 0,
            height: 0,
            accelerator: Accelerator::BruteForce,
            camera: Camera::default(),
            world: World::default(),
            render_method: RenderMethod::Pixels,
            algorithm: Algorithm::Whitted,
            threads: None,
            samples: 1,
            bvh_build_method: BvhBuildMethod::Sah,
        }
    }

    pub fn into_builder(self) -> RendererBuilder {
        RendererBuilder {
            width: self.width,
            height: self.height,
            accelerator: self.accelerator_type,
            camera: self.camera,
            world: self.world,
            render_method: self.render_method,
            algorithm: self.algorithm,
            threads: self.threads,
            samples: self.samples,
            bvh_build_method: self.bvh_build_method,
        }
    }

    pub fn render(&mut self, section: &Section) -> Vec<Color> {
        let trace: TraceFn = match self.algorithm {
            Algorithm::Whitted => whitted::trace_ray,
            Algorithm::PathTracing => path_tracing::trace_ray,
        };
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
    pub threads: Option<u32>,
    pub samples: u32,
    pub bvh_build_method: BvhBuildMethod,
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
        self
    }
    pub fn samples(&mut self, samples: u32) -> &mut RendererBuilder {
        self.samples = samples;
        self
    }
    pub fn bvh_build_method(&mut self, bvh_build_method: BvhBuildMethod) -> &mut RendererBuilder {
        self.bvh_build_method = bvh_build_method;
        self
    }
    pub fn build(&mut self) -> Renderer {
        Renderer::new(self)
    }
}
