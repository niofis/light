use rayon::prelude::*;
use std::f32::consts::PI;
use std::path::Path;

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

type AccStruct = BVH;
//type AccStruct = BruteForce;

pub struct World {
    width: u32,
    height: u32,
    camera: Camera,
    point_lights: Vec<Vector>,
    tracer: AccStruct,
    buffer: Vec<u8>,
    primitives: Vec<Primitive>,
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
        let tracer = AccStruct::new(&primitives);

        //println!("{:?} in bvh", tracer.stats());
        let buffer: Vec<u8> = vec![0; (4 * width * height) as usize];

        World {
            width,
            height,
            camera,
            point_lights,
            tracer,
            buffer,
            primitives,
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
        let tracer = AccStruct::new(&primitives);

        //println!("{:?} in bvh", tracer.stats());
        let buffer: Vec<u8> = vec![0; (4 * width * height) as usize];

        World {
            width,
            height,
            camera,
            point_lights,
            tracer,
            buffer,
            primitives,
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
        let primitives = vec![Primitive::new_triangle(
            Vector(-100.0, -100.0, 0.0),
            Vector(0.0, 100.0, 0.0),
            Vector(100.0, -100.0, 0.0),
            Material::Simple(Color(1.0, 1.0, 1.0)),
        )];

        let point_lights = vec![Vector(0.0, 0.0, -10.0)];

        //println!("{} total primitives", primitives.len());
        let tracer = AccStruct::new(&primitives);

        //println!("{:?} in bvh", tracer.stats());
        let buffer: Vec<u8> = vec![0; (4 * width * height) as usize];

        World {
            width,
            height,
            camera,
            point_lights,
            tracer,
            buffer,
            primitives,
        }
    }
    pub fn bunny(width: u32, height: u32) -> World {
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
        let mut primitives = Vec::new();

        primitives.push(Primitive::new_triangle(
            Vector(-800.0, -7.0, -800.0),
            Vector(0.0, -7.0, 800.0),
            Vector(800.0, -7.0, -800.0),
            Material::Simple(Color(1.0, 1.0, 1.0)),
        ));

        let bunny_obj = tobj::load_obj(&Path::new("models/bunny_res2.obj"));
        if bunny_obj.is_ok() == false {
            panic!("obj model is not valid!");
        }
        let (models, _) = bunny_obj.unwrap();
        let mesh_trs = Transform::combine(&vec![
            Transform::scale(120.0, 120.0, 120.0),
            Transform::translate(0.0, -10.0, 0.0),
        ]);

        for (_, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            for f in 0..mesh.indices.len() / 3 {
                let i = 3 * f;
                let x = 3 * mesh.indices[i] as usize;
                let pt1 = Vector(
                    -mesh.positions[x],
                    mesh.positions[x + 1],
                    mesh.positions[x + 2],
                );
                let x = 3 * mesh.indices[i + 1] as usize;
                let pt2 = Vector(
                    -mesh.positions[x],
                    mesh.positions[x + 1],
                    mesh.positions[x + 2],
                );
                let x = 3 * mesh.indices[i + 2] as usize;
                let pt3 = Vector(
                    -mesh.positions[x],
                    mesh.positions[x + 1],
                    mesh.positions[x + 2],
                );
                primitives.push(Primitive::new_triangle(
                    mesh_trs.apply(&pt1),
                    mesh_trs.apply(&pt3),
                    mesh_trs.apply(&pt2),
                    Material::white(),
                ));
            }
        }

        let point_lights = vec![Vector(0.0, 10.0, -20.0)];

        //println!("{} total primitives", primitives.len());
        let tracer = AccStruct::new(&primitives);

        //println!("{:?} in bvh", tracer.stats());
        let buffer: Vec<u8> = vec![0; (4 * width * height) as usize];

        World {
            width,
            height,
            camera,
            point_lights,
            tracer,
            buffer,
            primitives,
        }
    }

    pub fn render(&mut self) -> &[u8] {
        let height = self.height;
        let width = self.width;
        let camera = &self.camera;

        let pixels = (0..height * width)
            .into_par_iter()
            .map(|pixel| {
                let x = (pixel % width) as f32;
                let y = (pixel / width) as f32;
                let ray = camera.get_ray(x, y);

                trace_ray(&self, ray, 0)
            })
            .collect::<Vec<Color>>();

        let buffer = &mut self.buffer;
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
        let point_lights = &mut self.point_lights;
        point_lights[0] = rotation.apply(&point_lights[0]);
    }
}

fn trace_ray(world: &World, ray: Ray, depth: u8) -> Color {
    if depth > 10 {
        return Color(0.0, 0.0, 0.0);
    }

    match world.tracer.trace(&ray) {
        Some(prm_idxs) => {
            let closest = find_closest_primitive(world, &ray, &prm_idxs);
            match closest {
                Some((primitive, distance)) => {
                    let point = ray.point(distance);
                    let prm_material = match primitive {
                        Primitive::Sphere { material, .. } => material,
                        Primitive::Triangle { material, .. } => material,
                    };

                    match prm_material {
                        Material::Simple(_) => calculate_shading(world, primitive, &point),
                        Material::Reflective(_, idx) => {
                            let normal = primitive.normal(&point);
                            let ri = ray.1.unit();
                            let dot = ri.dot(&normal) * 2.0;
                            let new_dir = &ri - &(&normal * dot);
                            let reflected_ray = Ray::new(&point, &new_dir.unit());
                            (calculate_shading(world, primitive, &point) * (1.0 - idx))
                                + trace_ray(world, reflected_ray, depth + 1) * *idx
                        }
                    }
                }
                None => Color(0.0, 0.0, 0.0),
            }
        }
        None => Color(0.0, 0.0, 0.0),
    }
}

fn calculate_shading(world: &World, prm: &Primitive, point: &Vector) -> Color {
    let normal = prm.normal(point);
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

fn find_closest_primitive<'a>(
    world: &'a World,
    ray: &Ray,
    prm_indexes: &[usize],
) -> Option<(&'a Primitive, f32)> {
    let primitives = &world.primitives;
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
