use std::collections::HashMap;

use crate::Material;

use super::{light_source::LightSource, primitives::Primitive, solids::Solid};

#[derive(Clone, Debug, Default)]
pub struct World {
    pub lights: Vec<LightSource>,
    pub objects: Vec<Solid>,
    pub materials: HashMap<String, Material>,
}

#[derive(Clone, Debug, Default)]
pub struct WorldBuilder {
    lights: Vec<LightSource>,
    objects: Vec<Solid>,
    materials: HashMap<String, Material>,
}

impl WorldBuilder {
    pub fn add_light(&mut self, light: LightSource) -> &mut WorldBuilder {
        self.lights.push(light);
        self
    }
    pub fn add_object(&mut self, object: Solid) -> &mut WorldBuilder {
        self.objects.push(object);
        self
    }
    pub fn add_material(&mut self, name: &str, material: Material) -> &mut WorldBuilder {
        self.materials.insert(name.to_string(), material);
        self
    }
    pub fn build(&mut self) -> World {
        World {
            lights: self.lights.clone(),
            objects: self.objects.clone(),
            materials: self.materials.clone(),
        }
    }
}

impl World {
    pub fn builder() -> WorldBuilder {
        WorldBuilder::default()
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
