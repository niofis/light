use std::sync::Arc;

use crate::{color::Color, float::Float};

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
    pub fn red() -> Arc<Material> {
        Arc::new(Material::Diffuse(Color(1.0, 0.0, 0.0)))
    }
    pub fn green() -> Arc<Material> {
        Arc::new(Material::Diffuse(Color(0.0, 1.0, 0.0)))
    }
    pub fn blue() -> Arc<Material> {
        Arc::new(Material::Diffuse(Color(0.0, 0.0, 1.0)))
    }
    pub fn yellow() -> Arc<Material> {
        Arc::new(Material::Diffuse(Color(1.0, 1.0, 0.0)))
    }
    pub fn magenta() -> Arc<Material> {
        Arc::new(Material::Diffuse(Color(1.0, 0.0, 1.0)))
    }
    pub fn cyan() -> Arc<Material> {
        Arc::new(Material::Diffuse(Color(0.0, 1.0, 1.0)))
    }
    pub fn white() -> Arc<Material> {
        Arc::new(Material::Diffuse(Color(1.0, 1.0, 1.0)))
    }
    pub fn emissive_white() -> Arc<Material> {
        Arc::new(Material::Emissive(Color(1.0, 1.0, 1.0)))
    }
}
