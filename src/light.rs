use rayon::prelude::*;
use std::f32::consts::PI;
use std::sync::Arc;

mod bounding_box;
mod trace;
use bounding_box::*;
use trace::*;
mod bounding_volume_hierarchy;
use bounding_volume_hierarchy::*;
mod material;
use material::*;
mod color;
use color::*;
mod vector;
use vector::*;
mod ray;
use ray::*;
mod camera;
use camera::*;
mod primitive;
use primitive::*;
mod solids;
mod transform;
use transform::*;
mod brute_force;
use brute_force::*;

type AccStruct = BVH;

pub struct World {
    bpp: u32,
    width: u32,
    height: u32,
    camera: Camera,
    point_lights: Vec<Vector>,
    tracer: AccStruct,
}

impl World {
    pub fn demo(bpp: u32, width: u32, height: u32) -> World {
        let camera = Camera::new(
            Vector(0.0, 4.5, -75.0),
            Vector(-8.0, 9.0, -50.0),
            Vector(-8.0, 0.0, -50.0),
            Vector(8.0, 9.0, -50.0),
            width as f32,
            height as f32,
        );
        let mut primitives = vec![
            Primitive::Sphere {
                center: Vector(16.0, -2.0, 10.0),
                radius: 5.0,
                material: Material::Reflective(Color(0.0, 0.0, 1.0), 1.0),
            },
            Primitive::new_triangle(
                Vector(-8.0, 0.0, 0.0),
                Vector(-7.0, 2.0, 0.0),
                Vector(-6.0, 0.0, 0.0),
                Material::Simple(Color(0.0, 1.0, 0.0)),
            ),
        ];

        // cube thingy
        let cube_trs = vec![
            Transform::rotate(0.0, PI / 4.0, PI / 4.0),
            Transform::scale(3.0, 3.0, 3.0),
            Transform::translate(-10.0, -2.0, 0.0),
        ];
        let mut cube = solids::cube(&Transform::combine(&cube_trs));
        primitives.append(&mut cube);
        //cornell box
        let cornell_trs = vec![
            Transform::scale(42.0, 24.0, 50.0),
            Transform::translate(0.0, 5.0, 0.0),
        ];
        let mut cornell = solids::cornell_box(&Transform::combine(&cornell_trs));
        primitives.append(&mut cornell);

        //let primitives = cornell;

        let mut donut = solids::torus();
        primitives.append(&mut donut);

        //let primitives = donut;

        let point_lights = vec![Vector(-10.0, 10.0, -10.0)];

        println!("{} total", primitives.len());
        //let bvh = BVH::new(primitives[..].to_vec());
        let tracer = AccStruct::new(primitives);

        //println!("{:?} in bvh", bvh.stats());

        World {
            bpp,
            width,
            height,
            camera,
            point_lights,
            tracer,
        }
    }

    pub fn render(&self) -> Vec<u8> {
        let World {
            width,
            height,
            bpp,
            camera,
            point_lights,
            ..
        } = self;
        let pixels = (0..height * width)
            .into_par_iter()
            .map(|pixel| {
                let x = (pixel % width) as f32;
                let y = (pixel / width) as f32;
                let ray = camera.get_ray(x, y);

                //trace_ray(ray, primitives, point_lights, 0)

                trace_ray_bvh(ray, &self.tracer, point_lights, 0)
            })
            .collect::<Vec<Color>>();
        let mut buffer: Vec<u8> = vec![0; (bpp * width * height) as usize];

        let mut offset = 0;
        for pixel in pixels {
            let Color(r, g, b) = pixel;
            buffer[offset] = (b.min(1.0) * 255.99) as u8;
            buffer[offset + 1] = (g.min(1.0) * 255.99) as u8;
            buffer[offset + 2] = (r.min(1.0) * 255.99) as u8;
            buffer[offset + 3] = 255;
            offset = offset + 4;
        }
        buffer
    }
}

fn trace_ray_bvh(ray: Ray, tracer: &impl Trace, point_lights: &Vec<Vector>, depth: u8) -> Color {
    if depth > 10 {
        return Color(0.0, 0.0, 0.0);
    }

    match tracer.trace(&ray) {
        Some(prms) => {
            let closest = find_closest_primitive(&ray, &prms);
            match closest {
                Some((primitive, distance)) => {
                    let point = ray.point(distance);
                    let prm_material = match primitive {
                        Primitive::Sphere { material, .. } => material,
                        Primitive::Triangle { material, .. } => material,
                    };

                    match prm_material {
                        Material::Simple(_) => {
                            calculate_shading_bvh(primitive, &point, tracer, point_lights)
                        }
                        Material::Reflective(_, idx) => {
                            let normal = primitive.normal(&point);
                            let ri = ray.1.unit();
                            let dot = ri.dot(&normal) * 2.0;
                            let new_dir = &ri - &(&normal * dot);
                            let reflected_ray = Ray::new(&point, &new_dir.unit());
                            (calculate_shading_bvh(primitive, &point, tracer, point_lights)
                                * (1.0 - idx))
                                + trace_ray_bvh(reflected_ray, tracer, point_lights, depth + 1)
                                    * *idx
                        }
                    }
                }
                None => Color(0.0, 0.0, 0.0),
            }
        }
        None => Color(0.0, 0.0, 0.0),
    }
}

fn calculate_shading_bvh(
    prm: &Primitive,
    point: &Vector,
    tracer: &impl Trace,
    point_lights: &Vec<Vector>,
) -> Color {
    let normal = prm.normal(point);

    let incident_lights = point_lights.iter().filter_map(|light| {
        let direction = light - point;
        let ray = Ray::new(point, &(direction.unit()));
        match tracer.trace(&ray) {
            Some(prms) => {
                let closest = find_closest_primitive(&ray, &prms);
                let light_distance = direction.norm();
                match closest {
                    Some((_, dist)) => {
                        if dist > light_distance {
                            return Some(light);
                        } else {
                            return None;
                        }
                    }
                    None => Some(light),
                }
            }
            None => None,
        }
    });

    let prm_material = match prm {
        Primitive::Sphere { material, .. } => material,
        Primitive::Triangle { material, .. } => material,
    };

    let prm_color = match prm_material {
        Material::Simple(color) => color,
        Material::Reflective(color, _) => color,
    };

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

    Color(
        prm_color.0 * color_intensity.0,
        prm_color.1 * color_intensity.1,
        prm_color.2 * color_intensity.2,
    )
}

fn find_closest_primitive<'a>(
    ray: &Ray,
    primitives: &'a Vec<&Primitive>,
) -> Option<(&'a Primitive, f32)> {
    primitives
        .iter()
        .filter_map(|primitive| primitive.intersect(ray).map(|dist| (primitive, dist)))
        .fold(None, |closest, (pr, dist)| match closest {
            None => Some((pr, dist)),
            Some(res) if dist < res.1 => Some((pr, dist)),
            _ => closest,
        })
}
