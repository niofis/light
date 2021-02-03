use super::World;
use crate::light::Color;
use crate::light::Ray;
use crate::light::Trace;
use crate::light::Vector;
use rand::{Rng, SeedableRng};

fn random_dome<R: Rng>(rng: &mut R, normal: V3) -> V3 {
    rng.gen_iter::<(f32, f32, f32)>()
        .map(|(x, y, z)| {
            (V3 {
                x: x * 2. - 1.,
                y: y * 2. - 1.,
                z: z * 2. - 1.,
            })
            .unit()
        })
        .filter(|v| v.dot(&normal) >= 0.)
        .next()
        .unwrap()
}

fn path_tracing(
    world: &World,
    rng: &mut R,
    point: &Vector,
    normal: &Vector,
    samples: usize,
    depth: usize,
    max_depth: usize,
) -> Color {
    let mut acc = Color(0.0, 0.0, 0.0);
    for _ in [0..samples] {
        let ray = Ray::new(point, random_dome(rng, normal));
        match world.tracer.trace(&ray) {
            Some(prm_idxs) => {
                let light_distance = direction.norm();
                if find_shadow_primitive(world, &ray, &prm_idxs, light_distance) == false {
                    return Some(light);
                } else {
                    return None;
                }
            }
            None => None,
        }
    }
}

pub fn calculate(world: &World, point: &Vector, normal: &Vector) -> Color {
    let mut rng = rand::XorShiftRng::from_seed([
        point.0 * 100 as u32,
        point.1 * 100 as u32,
        point.2 * 100 as u32,
        42,
    ]);

    let incident_lights = world.point_lights.iter().filter_map(|light| {
        let direction = light - point;
        let ray = Ray::new(point, &(direction.unit()));
        match world.tracer.trace(&ray) {
            Some(prm_idxs) => {
                let light_distance = direction.norm();
                if find_shadow_primitive(world, &ray, &prm_idxs, light_distance) == false {
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
