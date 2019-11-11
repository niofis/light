use crate::light::color::*;

#[derive(Debug)]
pub enum Material {
    Simple(Color),
    Reflective(Color, f32),
}

impl Material {
    pub fn red() -> Material {
        Material::Simple(Color(1.0, 0.0, 0.0))
    }
    pub fn green() -> Material {
        Material::Simple(Color(0.0, 1.0, 0.0))
    }
    pub fn blue() -> Material {
        Material::Simple(Color(0.0, 0.0, 1.0))
    }
    pub fn yellow() -> Material {
        Material::Simple(Color(1.0, 1.0, 0.0))
    }
    pub fn magenta() -> Material {
        Material::Simple(Color(1.0, 0.0, 1.0))
    }
    pub fn cyan() -> Material {
        Material::Simple(Color(0.0, 1.0, 1.0))
    }
    pub fn white() -> Material {
        Material::Simple(Color(1.0, 1.0, 1.0))
    }
}
