use rand::prelude::ThreadRng;

use super::{color, primitive::Primitive, ray::Ray};
use crate::{Color, Material, Renderer, Vector};

pub fn trace_ray(renderer: &Renderer, rng: &mut ThreadRng, ray: &Ray, depth: u8) -> Color {
    let accelerator = &renderer.accelerator;
    if depth > 10 {
        return color::BLACK;
    }

    match accelerator.trace(&ray) {
        Some(prm_idxs) => {
            let closest = find_closest_primitive(&renderer, &ray, &prm_idxs);
            match closest {
                Some((primitive, distance)) => {
                    let point = ray.point(distance);
                    let prm_material = match primitive {
                        Primitive::Sphere { material, .. } => material,
                        Primitive::Triangle { material, .. } => material,
                    };

                    match prm_material {
                        Material::Simple(_) => calculate_shading(&renderer, &primitive, &point),
                        Material::Reflective(_, idx) => {
                            let normal = primitive.normal(&point);
                            let ri = ray.1.unit();
                            let dot = ri.dot(&normal) * 2.0;
                            let new_dir = &ri - &(&normal * dot);
                            let reflected_ray = Ray::new(&point, &new_dir.unit());
                            (calculate_shading(&renderer, &primitive, &point) * (1.0 - idx))
                                + trace_ray(renderer, rng, &reflected_ray, depth + 1) * *idx
                        }
                        Material::Emissive(color) => color.clone(),
                    }
                }
                None => color::BLACK,
            }
        }
        None => color::BLACK,
    }
}

fn calculate_shading(renderer: &Renderer, prm: &Primitive, point: &Vector) -> Color {
    let normal = prm.normal(point);
    let direct_lighting = calculate_direct_lighting(renderer, &point, &normal);

    let prm_material = match prm {
        Primitive::Sphere { material, .. } => material,
        Primitive::Triangle { material, .. } => material,
    };

    let prm_color = match prm_material {
        Material::Simple(color) => color,
        Material::Reflective(color, _) => color,
        Material::Emissive(color) => color,
    };

    Color(
        prm_color.0 * direct_lighting.0,
        prm_color.1 * direct_lighting.1,
        prm_color.2 * direct_lighting.2,
    )
}

fn find_closest_primitive<'a>(
    renderer: &'a Renderer,
    ray: &Ray,
    prm_indexes: &[usize],
) -> Option<(&'a Primitive, f32)> {
    let primitives = &renderer.primitives;
    prm_indexes
        .iter()
        .filter_map(|idx| {
            primitives[*idx]
                .intersect(ray)
                .map(|dist| (&primitives[*idx], dist))
        })
        .fold(None, |closest, (pr, dist)| match closest {
            None => Some((pr, dist)),
            Some(res) if dist < res.1 => Some((pr, dist)),
            _ => closest,
        })
}

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

fn calculate_direct_lighting(renderer: &Renderer, point: &Vector, normal: &Vector) -> Color {
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
