use crate::Color;
use crate::ilios::float::Float;

#[derive(Debug, Clone)]
pub enum Material {
    Diffuse(Color),
    Reflective(Color, Float),
    Emissive(Color),
    Refractive,
}

impl Default for Material {
    fn default() -> Material {
        Material::Diffuse(Color(1.0, 1.0, 1.0))
    }
}

impl Material {
    pub fn red() -> Material {
        Material::Diffuse(Color(1.0, 0.0, 0.0))
    }
    pub fn green() -> Material {
        Material::Diffuse(Color(0.0, 1.0, 0.0))
    }
    pub fn blue() -> Material {
        Material::Diffuse(Color(0.0, 0.0, 1.0))
    }
    pub fn yellow() -> Material {
        Material::Diffuse(Color(1.0, 1.0, 0.0))
    }
    pub fn magenta() -> Material {
        Material::Diffuse(Color(1.0, 0.0, 1.0))
    }
    pub fn cyan() -> Material {
        Material::Diffuse(Color(0.0, 1.0, 1.0))
    }
    pub fn white() -> Material {
        Material::Diffuse(Color(1.0, 1.0, 1.0))
    }
    pub fn emissive_white() -> Material {
        Material::Emissive(Color(1.0, 1.0, 1.0))
    }
}
