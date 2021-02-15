use super::{camera::Camera, light::Light, solid::Solid};

pub struct World {
    pub camera: Option<Camera>,
    pub lights: Option<Vec<Light>>,
    pub objects: Option<Vec<Solid>>,
}

impl World {
    pub fn build() -> World {
        World {
            camera: None,
            lights: None,
            objects: None,
        }
    }
    pub fn camera(&mut self, camera: Camera) -> &mut World {
        self.camera = Some(camera);
        self
    }
    pub fn lights(&mut self, lights: Vec<Light>) -> &mut World {
        self.lights = Some(lights);
        self
    }
    pub fn objects(&mut self, objects: Vec<Solid>) -> &mut World {
        self.objects = Some(objects);
        self
    }
    pub fn finish(self) -> World {
        self
    }
}
