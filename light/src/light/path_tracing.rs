use crate::{Color, Material, Renderer, Vector};
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;

use super::{
    closest_primitive::ClosestPrimitive,
    color,
    float::{Float, PI},
    primitive::Primitive,
    ray::Ray,
};

const MAX_DEPTH: u8 = 5;

fn trace_ray_internal(
    renderer: &Renderer,
    rng: &mut Xoshiro256PlusPlus,
    ray: &Ray,
    depth: u8,
) -> Color {
    if depth > MAX_DEPTH {
        return color::BLACK;
    }
    let accelerator = &renderer.accelerator;
    match accelerator.trace(ray) {
        Some(prm_idxs) => {
            let closest = find_closest_primitive(renderer, ray, &prm_idxs);
            match closest {
                Some(ClosestPrimitive {
                    primitive,
                    distance,
                }) => {
                    let point = ray.point(distance);
                    let prm_material = match primitive {
                        Primitive::Sphere { material, .. } => material,
                        Primitive::Triangle { material, .. } => material,
                    };

                    match prm_material {
                        Material::Simple(color) => {
                            let normal = primitive.normal(&point);
                            let new_dir = random_dome(rng, &normal);
                            let path_ray =
                                Ray::new(point, new_dir.unit().into(), Float::INFINITY, 1.0);
                            *color * trace_ray_internal(renderer, rng, &path_ray, depth + 1)
                        }
                        Material::Reflective(_, idx) => {
                            let normal = primitive.normal(&point);
                            let ri: Vector = ray.direction.into();
                            let dot = ri.dot(&normal) * 2.0;
                            let new_dir = ri - (normal * dot);
                            let reflected_ray =
                                Ray::new(point, new_dir.unit().into(), Float::INFINITY, 1.0);
                            trace_ray_internal(renderer, rng, &reflected_ray, depth + 1) * *idx
                        }
                        Material::Emissive(color) => *color,
                        Material::Refractive => {
                            let previous_index = ray.refraction_index;
                            let next_index = 1.52;
                            let mut normal = primitive.normal(&point);
                            let n = previous_index / next_index;
                            let dot = normal.dot(&ray.direction.into());
                            let ta = n * n * (1.0 - (dot * dot));
                            if previous_index == next_index {
                                normal = normal * -1.0;
                            }
                            let new_dir = (ray.direction * n) - normal * (n + (1.0 - ta).sqrt());

                            let refracted_ray =
                                Ray::new(point, new_dir.unit().into(), Float::INFINITY, 1.52);
                            trace_ray_internal(renderer, rng, &refracted_ray, depth + 1)
                        }
                    }
                }
                None => color::BLACK,
            }
        }
        None => color::BLACK,
    }
}

pub fn trace_ray(renderer: &Renderer, rng: &mut Xoshiro256PlusPlus, pixel: (u32, u32)) -> Color {
    let mut final_color = color::BLACK;
    let samples = renderer.samples;
    let (x, y) = pixel;
    for _ in 0..samples {
        let (nx, ny) = rng.gen::<(Float, Float)>();
        let ray = renderer.camera.get_ray(x as Float + nx, y as Float + ny);
        let sample_color = trace_ray_internal(renderer, rng, &ray, 1);
        final_color = final_color + sample_color
    }
    final_color / (samples as Float)
}

fn find_closest_primitive<'a>(
    renderer: &'a Renderer,
    ray: &Ray,
    prm_indexes: &[usize],
) -> Option<ClosestPrimitive<'a>> {
    let primitives = &renderer.primitives;
    prm_indexes
        .iter()
        .filter_map(|idx| {
            primitives[*idx]
                .intersect(ray)
                .map(|distance| ClosestPrimitive {
                    primitive: &primitives[*idx],
                    distance,
                })
        })
        .fold(None, |closest, next| match closest {
            None => Some(next),
            Some(current) if next.distance < current.distance => Some(next),
            _ => closest,
        })
}

fn rotate_vector(vector: &Vector, axis: &Vector, angle: Float) -> Vector {
    // vr = v * cos(angle) + (cross(axis, v))*sin(angle) + axis * dot(axis,v) * (1 - cos(angle))
    let (sin, cos) = angle.sin_cos();
    (vector * cos) + (axis.cross(vector) * sin) + (axis * axis.dot(vector) * (1.0 - cos))
}

fn random_dome(rng: &mut Xoshiro256PlusPlus, normal: &Vector) -> Vector {
    let (v, _) = normal.coordinate_system();
    let (r1, r2) = rng.gen::<(Float, Float)>();
    let first_rotation = 0.8 * r1 * PI / 2.0;
    let second_rotation = r2 * PI * 2.0;
    let nr = rotate_vector(normal, &v, first_rotation);
    rotate_vector(&nr, normal, second_rotation)

    // loop {
    //     let triple = rng.gen::<(Float, Float, Float)>();
    //     let new_vec = Vector(triple.0 * 2. - 1., triple.1 * 2. - 1., triple.2 * 2. - 1.).unit();
    //     if new_vec.dot(normal) >= 0. {
    //         return new_vec;
    //     }
    // }
}
