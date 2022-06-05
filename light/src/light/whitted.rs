use rand_xoshiro::Xoshiro256PlusPlus;

use super::{color, primitive::Primitive, ray::Ray};
use crate::{Color, Material, Point, Renderer, Vector};

fn inner_trace_ray(
    renderer: &Renderer,
    rng: &mut Xoshiro256PlusPlus,
    ray: &Ray,
    depth: u8,
) -> Color {
    if depth > 10 {
        return color::BLACK;
    }
    let accelerator = &renderer.accelerator;

    match accelerator.trace(ray) {
        Some(prm_idxs) => {
            let closest = find_closest_primitive(renderer, ray, &prm_idxs);
            match closest {
                Some((primitive, distance)) => {
                    let point = ray.point(distance);
                    let prm_material = match primitive {
                        Primitive::Sphere { material, .. } => material,
                        Primitive::Triangle { material, .. } => material,
                    };

                    match prm_material {
                        Material::Simple(_) => calculate_shading(renderer, primitive, &point),
                        Material::Reflective(_, idx) => {
                            let normal = primitive.normal(&point);
                            let ri = ray.direction.unit();
                            let dot = ri.dot(&normal) * 2.0;
                            let new_dir = ri - (normal * dot);
                            let reflected_ray = Ray::new(point, new_dir.unit(), f32::INFINITY);
                            (calculate_shading(renderer, primitive, &point) * (1.0 - idx))
                                + inner_trace_ray(renderer, rng, &reflected_ray, depth + 1) * *idx
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

pub fn trace_ray(renderer: &Renderer, rng: &mut Xoshiro256PlusPlus, pixel: (u32, u32)) -> Color {
    let (x, y) = pixel;
    let ray = renderer.camera.get_ray(x as f32, y as f32);

    inner_trace_ray(renderer, rng, &ray, 1)
}

fn calculate_shading(renderer: &Renderer, prm: &Primitive, point: &Point) -> Color {
    let normal = prm.normal(point);
    let direct_lighting = calculate_direct_lighting(renderer, point, &normal);

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

fn find_shadow_primitive(
    primitives: &[Primitive],
    ray: &Ray,
    prm_indexes: &[usize],
    max_dist: f32,
) -> bool {
    prm_indexes
        .iter()
        .filter_map(|idx| primitives[*idx].intersect(ray))
        .any(|dist| dist > 0.0001 && dist <= max_dist)
}

fn calculate_direct_lighting(renderer: &Renderer, point: &Point, normal: &Vector) -> Color {
    let incident_lights = renderer.world.lights.iter().filter_map(|ll| {
        let super::light_source::LightSource::Point(light, intensity) = ll;
        let direction = light - point;
        let unit_dir = direction.unit();
        let dot = normal.dot(&unit_dir);
        if dot <= 0.0 {
            return None;
        }

        let ray = Ray::new(*point, unit_dir, f32::INFINITY);
        match renderer.accelerator.trace(&ray) {
            Some(prm_idxs) => {
                let light_distance = direction.norm();
                if !find_shadow_primitive(&renderer.primitives, &ray, &prm_idxs, light_distance) {
                    let it = f32::max(0.0, (intensity - light_distance) / intensity);
                    Some(Color(1.0, 1.0, 1.0) * dot * it)
                } else {
                    None
                }
            }
            None => None,
        }
    });

    incident_lights.fold(Color(0.0, 0.0, 0.0), |acc, col| acc + col)
}
