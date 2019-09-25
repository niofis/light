use rayon::prelude::*;
use std::ops;

struct Color(f32, f32, f32); //r,g,b

#[derive(Clone, Copy, Debug)]
struct Vector(f32, f32, f32); //x,y,z

#[derive(Clone, Copy, Debug)]
struct Ray(Vector, Vector); //origin, direction

struct Camera {
    eye: Vector,
    left_top: Vector,
    delta_right: Vector,
    delta_down: Vector,
}

#[derive(Clone, Copy, Debug)]
enum Primitive {
    Sphere(Vector, f32),
    Triangle(Vector, Vector, Vector, Vector),
}

pub struct World {
    bpp: u32,
    width: u32,
    height: u32,
    camera: Camera,
    primitives: Vec<Primitive>,
}

struct Hit {
    normal: Vector,
    distance: f32,
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 + x1, y0 + y1, z0 + z1)
    }
}
/*
impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 + x1, y0 + y1, z0 + z1)
    }
}
*/
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 - x1, y0 - y1, z0 - z1)
    }
}
/*
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 - x1, y0 - y1, z0 - z1)
    }
}
*/
impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        let Vector(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}
/*
impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        let Vector(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}
*/
impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Vector {
        let Vector(x, y, z) = self;
        Vector(x / rhs, y / rhs, z / rhs)
    }
}

impl Vector {
    fn dot(self, rhs: Vector) -> f32 {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        x0 * x1 + y0 * y1 + z0 * z1
    }
    fn norm(self) -> f32 {
        self.dot(self).sqrt()
    }
    fn unit(self) -> Vector {
        self / self.norm()
    }
}

impl Camera {
    fn new(
        eye: Vector,
        left_top: Vector,
        left_bottom: Vector,
        right_top: Vector,
        width: f32,
        height: f32,
    ) -> Camera {
        let delta_right = (right_top - left_top) / width;
        let delta_down = (left_bottom - left_top) / height;

        Camera {
            eye,
            left_top,
            delta_right,
            delta_down,
        }
    }

    fn get_ray(&self, x: f32, y: f32) -> Ray {
        let Camera {
            left_top,
            delta_right,
            delta_down,
            eye,
        } = self;

        let origin = *left_top + (*delta_right * x) + (*delta_down * y);
        let direction = origin - *eye;

        Ray(origin, direction)
    }
}

impl World {
    pub fn demo(bpp: u32, width: u32, height: u32) -> World {
        let camera = Camera::new(
            Vector(0.0, 4.5, 75.0),
            Vector(-8.0, 9.0, 50.0),
            Vector(-8.0, 0.0, 50.0),
            Vector(8.0, 9.0, 50.0),
            width as f32,
            height as f32,
        );
        let primitives = vec![Primitive::Sphere(Vector(0.0, 0.0, 0.0), 2.0)];
        World {
            bpp,
            width,
            height,
            camera,
            primitives,
        }
    }

    pub fn render(&self) -> Vec<u8> {
        let World {
            width,
            height,
            bpp,
            camera,
            primitives,
        } = self;
        let pixels = (0..height * width).map(|pixel| {
            let x = (pixel % width) as f32;
            let y = (pixel / width) as f32;
            let ray = camera.get_ray(x, y);
            trace(primitives, ray)
        });
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

fn sphere_intersect(sphere: (Vector, f32), ray: Ray) -> Option<f32> {
    let (center, radius) = sphere;
    let Ray(origin, direction) = ray;
    let oc = origin - center;
    let a = direction.dot(direction);
    let b = oc.dot(direction);
    let c = oc.dot(oc) - radius * radius;
    let dis = b * b - a * c;

    if dis > 0.0 {
        let e = dis.sqrt();

        let distance = (-b - e) / a;
        if distance > 0.007 {
            return Some(distance);
        }

        let distance = (-b + e) / a;
        if distance > 0.007 {
            return Some(distance);
        }
    }
    None
}

fn intersect(primitive: &Primitive, ray: Ray) -> Option<f32> {
    match primitive {
        Primitive::Sphere(center, radius) => sphere_intersect((*center, *radius), ray),
        _ => None,
    }
}

/*
fn render() -> Vec<u8> {
    let bpp = 4;
    let mut buffer: Vec<u8> = vec![0; bpp * WIDTH * HEIGHT];
    let x = 400;
    let y = 300;
    let offset = (y * WIDTH + x) * bpp;
    buffer[offset] = 255; //B
    buffer[offset + 1] = 0; //G
    buffer[offset + 2] = 0; //R
    buffer[offset + 3] = 0; //A? ignored
    buffer
}
*/
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

fn trace(primitives: &Vec<Primitive>, ray: Ray) -> Color {
    let hit: bool = primitives
        .iter()
        .map(|primitive| intersect(primitive, ray))
        .any(|hit| match hit {
            Some(distance) => true,
            None => false,
        });

    if hit {
        Color(1.0, 0.0, 0.0)
    } else {
        Color(0.0, 0.0, 0.0)
    }
}
