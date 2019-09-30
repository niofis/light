use rayon::prelude::*;
use std::ops;

#[derive(Clone, Copy, Debug)]
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
    Sphere {
        center: Vector,
        radius: f32,
        color: Color,
    },
    Triangle {
        origin: Vector,
        edge1: Vector,
        edge2: Vector,
        normal: Vector,
        color: Color,
    },
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

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 - x1, y0 - y1, z0 - z1)
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        let Vector(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}

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
    fn cross(self, rhs: Vector) -> Vector {
        let Vector(x1, y1, z1) = self;
        let Vector(x2, y2, z2) = rhs;
        Vector(y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
    }
}

impl Primitive {
    fn new_triangle(pt1: Vector, pt2: Vector, pt3: Vector, color: Color) -> Primitive {
        let edge1 = pt2 - pt1;
        let edge2 = pt3 - pt1;
        let normal = edge1.cross(edge2).unit();
        Primitive::Triangle {
            origin: pt1,
            edge1,
            edge2,
            normal,
            color,
        }
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
        let primitives = vec![
            Primitive::Sphere {
                center: Vector(0.0, 0.0, 0.0),
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

fn triangle_intersect(triangle: (Vector, Vector, Vector), ray: Ray) -> Option<f32> {
    let (v0, edge1, edge2) = triangle;
    let Ray(origin, direction) = ray;
    let pvec = direction.cross(edge2);

    let det = edge1.dot(pvec);
    //No culling version
    if det > -0.007 && det < 0.007 {
        return None;
    }

    let inv_det = 1.0 / det;

    let tvec = origin - v0;

    let u = tvec.dot(pvec) * inv_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let qvec = tvec.cross(edge1);

    let v = direction.dot(qvec) * inv_det;
    if v < 0.0 || (u + v) > 1.007 {
        //add EPSILON to offset small precision errors
        return None;
    }

    let t = edge2.dot(qvec) * inv_det;

    if t > 0.007 {
        return Some(t);
    }

    None
}

fn intersect(primitive: &Primitive, ray: Ray) -> Option<f32> {
    match primitive {
        Primitive::Sphere { center, radius, .. } => sphere_intersect((*center, *radius), ray),
        Primitive::Triangle {
            origin,
            edge1,
            edge2,
            ..
        } => triangle_intersect((*origin, *edge1, *edge2), ray),
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
    let closest = primitives
        .iter()
        .filter_map(|primitive| intersect(primitive, ray).map(|dist| (primitive, dist)))
        .fold(None, |closest, (pr, dist)| match closest {
            None => Some((pr, dist)),
            Some(res) if dist < res.1 => Some((pr, dist)),
            _ => closest,
        });

    match closest {
        Some((primitive, _)) => match primitive {
            Primitive::Sphere { color, .. } => *color,
            Primitive::Triangle { color, .. } => *color,
        },
        None => Color(0.0, 0.0, 0.0),
    }
}
