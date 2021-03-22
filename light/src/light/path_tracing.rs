use rand::{prelude::ThreadRng, Rng};

use crate::{Color, Material, Renderer, Vector};

use super::{color, primitive::Primitive, ray::Ray};

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
                        Material::Simple(color) => {
                            let normal = primitive.normal(&point);
                            let new_dir = random_dome(rng, &normal);
                            let path_ray = Ray::new(&point, &new_dir);
                            color.clone() * trace_ray(renderer, rng, &path_ray, depth + 1)
                        }
                        Material::Reflective(_, idx) => {
                            let normal = primitive.normal(&point);
                            let ri = ray.1.unit();
                            let dot = ri.dot(&normal) * 2.0;
                            let new_dir = &ri - &(&normal * dot);
                            let reflected_ray = Ray::new(&point, &new_dir.unit());
                            trace_ray(renderer, rng, &reflected_ray, depth + 1) * *idx
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

fn random_dome(rng: &mut ThreadRng, normal: &Vector) -> Vector {
    loop {
        let triple = rng.gen::<(f32, f32, f32)>();
        let new_vec = Vector(triple.0 * 2. - 1., triple.1 * 2. - 1., triple.2 * 2. - 1.).unit();
        if new_vec.dot(&normal) >= 0. {
            return new_vec;
        }
    }
    // rng.sample_iter::<(f32, f32, f32)>(rand::distributions::Standard)
    //     .map(|(x, y, z)| {
    //         (Vector {
    //             x: x * 2. - 1.,
    //             y: y * 2. - 1.,
    //             z: z * 2. - 1.,
    //         })
    //         .unit()
    //     })
    //     .filter(|v| v.dot(&normal) >= 0.)
    //     .next()
    //     .unwrap()
}
