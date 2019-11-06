use crate::light::color::*;

pub enum Material {
    Simple(Color),
    Reflective(Color, f32),
}
