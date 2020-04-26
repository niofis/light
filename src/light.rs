use rayon::prelude::*;
use std::f32::consts::PI;

mod bounding_box;
mod trace;
use trace::*;
mod brute_force;
use brute_force::*;
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

//type AccStruct = BVH;
type AccStruct = BruteForce;

pub struct World {
    width: u32,
    height: u32,
    camera: Camera,
    point_lights: Vec<Vector>,
    tracer: AccStruct,
    buffer: Vec<u8>,
}

impl World {
    pub fn demo(width: u32, height: u32) -> World {
        let gw = 20.0;
        let gh = 15.0;
        let camera = Camera::new(
            Vector(0.0, gh / 2.0, -75.0),
            Vector(-gw / 2.0, gh, -50.0),
            Vector(-gw / 2.0, 0.0, -50.0),
            Vector(gw / 2.0, gh, -50.0),
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
            Transform::scale(42.0, 30.0, 50.0),
            Transform::translate(0.0, 7.5, 0.0),
        ];
        let mut cornell = solids::cornell_box(&Transform::combine(&cornell_trs));
        primitives.append(&mut cornell);

        //this is a donut
        let donut_trs = vec![Transform::rotate(PI / -4.0, 0.0, 0.0)];
        let mut donut = solids::torus(1.5, 4.0, 30, 50, &Transform::combine(&donut_trs));
        primitives.append(&mut donut);

        let point_lights = vec![Vector(-10.0, 10.0, -10.0)];

        //println!("{} total primitives", primitives.len());
        let tracer = AccStruct::new(primitives);

        //println!("{:?} in bvh", tracer.stats());
        let buffer: Vec<u8> = vec![0; (4 * width * height) as usize];

        World {
            width,
            height,
            camera,
            point_lights,
            tracer,
            buffer,
        }
    }
    
    pub fn demo2(width: u32, height: u32) -> World {
        let gw = 20.0;
        let gh = 15.0;
        let camera = Camera::new(
            Vector(0.0, gh / 2.0, -75.0),
            Vector(-gw / 2.0, gh, -50.0),
            Vector(-gw / 2.0, 0.0, -50.0),
            Vector(gw / 2.0, gh, -50.0),
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
                Vector(-800.0, -7.0, -800.0),
                Vector(0.0, -7.0, 800.0),
                Vector(800.0, -7.0, -800.0),
                Material::Simple(Color(1.0, 1.0, 1.0)),
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
        
        //this is a donut
        let donut_trs = vec![Transform::rotate(PI / -4.0, 0.0, 0.0)];
        let mut donut = solids::torus(1.5, 4.0, 30, 50, &Transform::combine(&donut_trs));
        primitives.append(&mut donut);

        let sphere_trs = vec![Transform::translate(-16.0, -2.0, 10.0)];
        let mut sphere = solids::sphere(2.0, 20, &Transform::combine(&sphere_trs));
        primitives.append(&mut sphere);



        let point_lights = vec![Vector(-10.0, 10.0, -10.0)];

        //println!("{} total primitives", primitives.len());
        let tracer = AccStruct::new(primitives);

        //println!("{:?} in bvh", tracer.stats());
        let buffer: Vec<u8> = vec![0; (4 * width * height) as usize];

        World {
            width,
            height,
            camera,
            point_lights,
            tracer,
            buffer,
        }
    }
    
    pub fn shader_bench(width: u32, height: u32) -> World {
        let gw = 20.0;
        let gh = 15.0;
        let camera = Camera::new(
            Vector(0.0, gh / 2.0, -75.0),
            Vector(-gw / 2.0, gh, -50.0),
            Vector(-gw / 2.0, 0.0, -50.0),
            Vector(gw / 2.0, gh, -50.0),
            width as f32,
            height as f32,
        );
        let mut primitives = vec![
            Primitive::new_triangle(
                Vector(-100.0, -100.0, 0.0),
                Vector(0.0, 100.0, 0.0),
                Vector(100.0, -100.0, 0.0),
                Material::Simple(Color(1.0, 1.0, 1.0)),
            ),
        ];

        let point_lights = vec![Vector(0.0, 0.0, -10.0)];

        //println!("{} total primitives", primitives.len());
        let tracer = AccStruct::new(primitives);

        //println!("{:?} in bvh", tracer.stats());
        let buffer: Vec<u8> = vec![0; (4 * width * height) as usize];

        World {
            width,
            height,
            camera,
            point_lights,
            tracer,
            buffer,
        }
    }

    pub fn render(&mut self) -> &[u8] {
        let height = self.height;
        let width = self.width;
        let camera = &self.camera;
        let point_lights = &self.point_lights;
        let tracer = &self.tracer;
        let buffer = &mut self.buffer;

        let pixels = (0..height * width)
            .into_iter()
            .map(|pixel| {
                let x = (pixel % width) as f32;
                let y = (pixel / width) as f32;
                let ray = camera.get_ray(x, y);

                trace_ray(ray, tracer, point_lights, 0)
            })
            .collect::<Vec<Color>>();

        let mut offset = 0;
        for pixel in pixels {
            let Color(r, g, b) = pixel;
            buffer[offset] = if b > 1.0 { 255 } else { (b * 255.99) as u8 };
            buffer[offset + 1] = if g > 1.0 { 255 } else { (g * 255.99) as u8 };
            buffer[offset + 2] = if r > 1.0 { 255 } else { (r * 255.99) as u8 };
            offset = offset + 4;
        }
        &self.buffer
    }

    pub fn rotate_camera(&mut self, rads: f32) {
        let rotation = Transform::rotate(0.0, rads, 0.0);
        let mut camera = &mut self.camera;
        camera.left_top = rotation.apply(&camera.left_top);
        camera.delta_down = rotation.apply(&camera.delta_down);
        camera.delta_right = rotation.apply(&camera.delta_right);
        camera.eye = rotation.apply(&camera.eye);
    }

    pub fn rotate_light(&mut self, rads: f32) {
        let rotation = Transform::rotate(0.0, rads, 0.0);
        let mut point_lights = &mut self.point_lights;
        point_lights[0] = rotation.apply(&point_lights[0]);
    }
}

fn trace_ray(ray: Ray, tracer: &impl Trace, point_lights: &Vec<Vector>, depth: u8) -> Color {
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
                            calculate_shading(primitive, &point, tracer, point_lights)
                        }
                        Material::Reflective(_, idx) => {
                            let normal = primitive.normal(&point);
                            let ri = ray.1.unit();
                            let dot = ri.dot(&normal) * 2.0;
                            let new_dir = &ri - &(&normal * dot);
                            let reflected_ray = Ray::new(&point, &new_dir.unit());
                            (calculate_shading(primitive, &point, tracer, point_lights)
                                * (1.0 - idx))
                                + trace_ray(reflected_ray, tracer, point_lights, depth + 1) * *idx
                        }
                    }
                }
                None => Color(0.0, 0.0, 0.0),
            }
        }
        None => Color(0.0, 0.0, 0.0),
    }
}

fn calculate_shading(
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
                let light_distance = direction.norm();
                if find_shadow_primitive(&ray, &prms, light_distance) == false {
                    return Some(light);
                } else {
                    return None;
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

fn find_shadow_primitive<'a>(
    ray: &Ray,
    primitives: &'a Vec<&Primitive>,
    max_dist: f32
) -> bool {
    primitives
        .iter()
        .filter_map(|primitive| primitive.intersect(ray).map(|dist|  dist))
        .any(|dist| dist > 0.0001 && dist <= max_dist)
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
