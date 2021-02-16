use super::{camera::Camera, light::Light, primitive::Primitive, solid::Solid};

pub struct World {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Solid>,
}

impl World {
    pub fn default() -> World {
        World::build()
    }
    pub fn build() -> World {
        World {
            camera: Camera::default(),
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }
    pub fn camera(&mut self, camera: Camera) -> &mut World {
        self.camera = camera;
        self
    }
    pub fn lights(&mut self, lights: Vec<Light>) -> &mut World {
        self.lights = lights;
        self
    }
    pub fn objects(&mut self, objects: Vec<Solid>) -> &mut World {
        self.objects = objects;
        self
    }
    pub fn finish(self) -> World {
        self
    }
    pub fn primitives(&self) -> Vec<Primitive> {
        self.objects
            .iter()
            .map(|obj| obj.primitives())
            .fold(Vec::new(), |acc, prms: Vec<Primitive>| {
                acc.into_iter().chain(prms.into_iter()).collect()
            })
    }
}
