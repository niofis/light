//use rayon::prelude::*;

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

pub struct World {
    bpp: u32,
    width: u32,
    height: u32,
    camera: Camera,
    primitives: Vec<Primitive>,
    point_lights: Vec<Vector>,
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
        let primitives = vec![
            Primitive::Sphere {
                center: Vector(0.0, -1_000_000.0, 0.0),
                radius: 1_000_000.0,
                color: Color(1.0, 1.0, 1.0),
            },
            Primitive::Sphere {
                center: Vector(0.0, 2.0, 0.0),
                radius: 2.0,
                color: Color(0.0, 0.0, 1.0),
            },
            Primitive::new_triangle(
                Vector(-8.0, 0.0, 0.0),
                Vector(-7.0, 2.0, 0.0),
                Vector(-6.0, 0.0, 0.0),
                Color(0.0, 1.0, 0.0),
            ),
        ];
        let point_lights = vec![Vector(10.0, 10.0, -10.0)];
        World {
            bpp,
            width,
            height,
            camera,
            primitives,
            point_lights,
        }
    }

    pub fn render(&self) -> Vec<u8> {
        let World {
            width,
            height,
            bpp,
            camera,
            primitives,
            point_lights,
        } = self;
        let pixels = (0..height * width)
            //.into_par_iter()
            .map(|pixel| {
                let x = (pixel % width) as f32;
                let y = (pixel / width) as f32;
                let ray = camera.get_ray(x, y);

                let closest = find_closest_primitive(&ray, primitives);

                match closest {
                    Some((primitive, distance)) => {
                        calculate_shading(primitive, ray.point(distance), primitives, point_lights)
                    }
                    None => Color(0.0, 0.0, 0.0),
                }
            });
        // .collect::<Vec<Color>>();
        let mut buffer: Vec<u8> = vec![0; (bpp * width * height) as usize];

        let mut offset = 0;
        for pixel in pixels {
            let Color(r, g, b) = pixel;
            buffer[offset] = (b * 255.99) as u8;
            buffer[offset + 1] = (g * 255.99) as u8;
            buffer[offset + 2] = (r * 255.99) as u8;
            buffer[offset + 3] = 255;
            offset = offset + 4;
        }
        buffer
    }
}

/*
fn render1() -> Vec<u8> {
    let bpp = 4;
    let pixels = (0..HEIGHT * WIDTH).map(|pixel| {
        //let x = pixel % WIDTH;
        //let y = pixel / WIDTH;

        Color(1.0, 0.0, 0.0)
    });

    let mut buffer = Vec::new();
    let buffer: Vec<u8> = pixels.fold(buffer, |mut acc, pixel| {
        let Color(r, g, b) = pixel;
        acc.push((b * 255.99) as u8);
        acc.push((g * 255.99) as u8);
        acc.push((r * 255.99) as u8);
        acc.push(255 as u8);
        acc
    });
    buffer
}

fn render2() -> Vec<u8> {
    let bpp = 4;
    let pixels: Vec<Color> = (0..HEIGHT * WIDTH)
        .map(|pixel| {
            //let x = pixel % WIDTH;
            //let y = pixel / WIDTH;

            Color(1.0, 0.0, 0.0)
        })
        .collect();
    let mut buffer: Vec<u8> = vec![0; bpp * WIDTH * HEIGHT];
    for pixel in 0..HEIGHT * WIDTH {
        let Color(r, g, b) = pixels[pixel];
        let x = pixel % WIDTH;
        let y = pixel / WIDTH;

        let offset = (y * WIDTH + x) * bpp;
        buffer[offset] = (b * 255.99) as u8;
        buffer[offset + 1] = (g * 255.99) as u8;
        buffer[offset + 2] = (r * 255.99) as u8;
        buffer[offset + 3] = 255;
    }

    buffer
}
*/

fn find_closest_primitive<'a>(
    ray: &Ray,
    primitives: &'a Vec<Primitive>,
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

fn calculate_shading(
    prm: &Primitive,
    point: Vector,
    primitives: &Vec<Primitive>,
    point_lights: &Vec<Vector>,
) -> Color {
    let normal = prm.normal(point);

    let incident_lights = point_lights.iter().filter_map(|light| {
        let ray = Ray(point, *light);
        let closest = find_closest_primitive(&ray, primitives);
        let light_distance = (*light - point).norm();
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
    });

    let prm_color = match prm {
        Primitive::Sphere { color, .. } => color,
        Primitive::Triangle { color, .. } => color,
    };

    let color_intensity = incident_lights
        .map(|light| {
            let dot = normal.dot(&light.unit());
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
