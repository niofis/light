use crate::Renderer;

use super::{color::Color, primitive::Primitive, ray::Ray, vector::Vector};

fn find_shadow_primitive<'a>(
    primitives: &Vec<Primitive>,
    ray: &Ray,
    prm_indexes: &Vec<usize>,
    max_dist: f32,
) -> bool {
    prm_indexes
        .iter()
        .filter_map(|idx| primitives[*idx].intersect(ray).map(|dist| dist))
        .any(|dist| dist > 0.0001 && dist <= max_dist)
}

pub fn calculate(renderer: &Renderer, point: &Vector, normal: &Vector) -> Color {
    let incident_lights = renderer.world.lights.iter().filter_map(|ll| {
        let light = match ll {
            super::light::Light::Point(pos) => pos,
        };
        let direction = light - point;
        let ray = Ray::new(point, &(direction.unit()));
        match renderer.accelerator.trace(&ray) {
            Some(prm_idxs) => {
                let light_distance = direction.norm();
                if find_shadow_primitive(&renderer.primitives, &ray, &prm_idxs, light_distance)
                    == false
                {
                    return Some(light);
                } else {
                    return None;
                }
            }
            None => None,
        }
    });

    let color_intensity = incident_lights
        .map(|light| {
            let dot = normal.dot(&(light - &point).unit());
            if dot < 0.0 {
                return Color(0.0, 0.0, 0.0);
            } else {
                return Color(1.0, 1.0, 1.0) * dot;
            }
        })
        .fold(Color(0.0, 0.0, 0.0), |acc, col| acc + col);

    color_intensity
}
