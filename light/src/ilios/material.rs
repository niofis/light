use crate::ilios::float::Float;
use crate::Color;

#[derive(Debug, Clone)]
pub enum Material {
    Diffuse(Color),
    Reflective(Color, Float),
    Emissive(Color),
    Refractive,
}

impl Material {
    pub fn default() -> Material {
        Material::Diffuse(Color(1.0, 1.0, 1.0))
    }
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
}
