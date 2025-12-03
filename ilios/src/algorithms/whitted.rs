use ilios_types::{
    color::{self, BLACK, Color},
    float::Float,
    geometry::{Normal, Point, Triangle, Vector},
    light_source::LightSource,
    material::Material,
    ray::Ray,
};

use crate::{
    Renderer,
    {closest_primitive::ClosestPrimitive, geometry::PackedTriangles, rng::Rng, simd},
};

fn inner_trace_ray(renderer: &Renderer, ray: &Ray, depth: u8) -> Color {
    if depth > 10 {
        return color::BLACK;
    }
    let accelerator = &renderer.accelerator;

    match accelerator.trace(ray) {
        Some(prms) => {
            let closest = find_closest_primitive(&prms, ray);
            match closest {
                Some(ClosestPrimitive {
                    primitive,
                    distance,
                }) => {
                    let point = ray.point(distance);
                    let prm_material = &primitive.material;

                    match prm_material.as_ref() {
                        Material::Diffuse(_) => calculate_shading(renderer, primitive, &point),
                        Material::Reflective(_, idx) => {
                            let normal: Vector = primitive.normal().into();
                            let ri: Vector = ray.direction.into();
                            let dot = ri.dot(&normal) * 2.0;
                            let new_dir = ri - (normal * dot);
                            let reflected_ray =
                                Ray::new(point, new_dir.unit(), Float::INFINITY, 1.0);
                            (calculate_shading(renderer, primitive, &point) * (1.0 - idx))
                                + inner_trace_ray(renderer, &reflected_ray, depth + 1) * *idx
                        }
                        Material::Emissive(color) => *color,
                        Material::Refractive => {
                            let current_index = 1.52; //assume it is glass
                            let previous_index = 1.0;
                            let normal = primitive.normal();
                            let n = current_index / previous_index;
                            let dot = normal.dot(&ray.direction.into());
                            let ta = n * n * (1.0 - (dot * dot));
                            let new_dir = ((ray.direction * n) - normal * (1.0 - ta).sqrt()).unit();
                            let refracted_ray = Ray::new(point, new_dir, Float::INFINITY, 1.0);
                            inner_trace_ray(renderer, &refracted_ray, depth + 1)
                        }
                    }
                }
                None => color::BLACK,
            }
        }
        None => color::BLACK,
    }
}

pub fn trace_ray(renderer: &Renderer, _rng: &mut dyn Rng, pixel: (u32, u32)) -> Color {
    let (x, y) = pixel;
    let ray = renderer.camera.get_ray(x as Float, y as Float);

    inner_trace_ray(renderer, &ray, 1)
}

fn calculate_shading(renderer: &Renderer, prm: &Triangle, point: &Point) -> Color {
    let normal = prm.normal();
    let direct_lighting = calculate_direct_lighting(renderer, point, &normal);

    let prm_material = &prm.material;

    let prm_color = match prm_material.as_ref() {
        Material::Diffuse(color) => color,
        Material::Reflective(color, _) => color,
        Material::Emissive(color) => color,
        Material::Refractive => &BLACK,
    };

    Color(
        prm_color.0 * direct_lighting.0,
        prm_color.1 * direct_lighting.1,
        prm_color.2 * direct_lighting.2,
    )
}

fn find_closest_primitive<'a>(
    primitives: &[&'a PackedTriangles],
    ray: &Ray,
) -> Option<ClosestPrimitive<'a>> {
    primitives
        .iter()
        .filter_map(|primitive| {
            primitive.intersect(&ray).map(|distances| {
                let mut closest_distance = f32::MAX;
                let mut closest_idx = 0;

                for idx in 0..4 {
                    let distance = simd::get(distances, idx);
                    if distance > 0.0 && distance < closest_distance {
                        closest_distance = distance;
                        closest_idx = idx;
                    }
                }

                ClosestPrimitive {
                    primitive: primitive.triangles[closest_idx].as_ref(),
                    distance: closest_distance,
                }
            })
        })
        .fold(None, |closest, next| match closest {
            None => Some(next),
            Some(current) if next.distance < current.distance => Some(next),
            _ => closest,
        })
}

fn find_shadow_primitive(primitives: &[&PackedTriangles], ray: &Ray, max_dist: Float) -> bool {
    primitives
        .iter()
        .filter_map(|primitive| {
            primitive.intersect(&ray).map(|distances| {
                let mut closest_distance = f32::MAX;
                let mut closest_idx = 0;

                for idx in 0..4 {
                    let distance = simd::get(distances, idx);
                    if distance > 0.0 && distance < closest_distance {
                        closest_distance = distance;
                        closest_idx = idx;
                    }
                }

                ClosestPrimitive {
                    primitive: primitive.triangles[closest_idx].as_ref(),
                    distance: closest_distance,
                }
            })
        })
        .any(|p| p.distance > 0.0001 && p.distance <= max_dist)
}

fn calculate_direct_lighting(renderer: &Renderer, point: &Point, normal: &Normal) -> Color {
    let incident_lights = renderer.world.lights.iter().filter_map(|ll| {
        let LightSource::Point(light, intensity) = ll;
        let direction = light - point;
        let unit_dir = direction.unit();
        let dot = normal.dot(&unit_dir.into());
        if dot <= 0.0 {
            return None;
        }

        let ray = Ray::new(*point, unit_dir, Float::INFINITY, 1.0);
        match renderer.accelerator.trace(&ray) {
            Some(prms) => {
                let light_distance = direction.norm();
                if !find_shadow_primitive(&prms, &ray, light_distance) {
                    let it = Float::max(0.0, (intensity - light_distance) / intensity);
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
