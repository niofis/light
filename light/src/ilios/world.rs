use std::collections::HashMap;

use crate::Material;

use super::{light_source::LightSource, primitives::Primitive, solids::Solid};

#[derive(Clone, Debug, Default)]
pub struct World {
    pub lights: Vec<LightSource>,
    pub objects: Vec<Solid>,
    pub materials: HashMap<String, Material>,
}

impl World {
    pub fn build() -> World {
        World {
            lights: Vec::new(),
            objects: Vec::new(),
            materials: HashMap::new(),
        }
    }
    pub fn lights(mut self, lights: Vec<LightSource>) -> World {
        self.lights = lights;
        self
    }
    pub fn objects(mut self, objects: Vec<Solid>) -> World {
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
                acc.into_iter().chain(prms).collect()
            })
    }
}
