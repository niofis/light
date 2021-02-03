use super::World;
use crate::light::Color;
use crate::light::Ray;
use crate::light::Trace;
use crate::light::Vector;

fn find_shadow_primitive<'a>(
    world: &World,
    ray: &Ray,
    prm_indexes: &[usize],
    max_dist: f32,
) -> bool {
    let primitives = &world.primitives;
    prm_indexes
        .iter()
        .filter_map(|idx| primitives[*idx].intersect(ray).map(|dist| dist))
        .any(|dist| dist > 0.0001 && dist <= max_dist)
}

pub fn calculate(world: &World, point: &Vector, normal: &Vector) -> Color {
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
