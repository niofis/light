extern crate sdl2;
extern crate time;
extern crate rayon;
extern crate rand;

use std::ops;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
// use std::time::Duration;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

const WIDTH: usize = 427;
const HEIGHT: usize = 240;
const SAMPLES: u32 = 1;
const MAXDEPTH: u32 = 5;

#[derive(Copy, Clone, Debug)]
struct V3 {
    x: f32,
    y: f32,
    z: f32
}

impl ops::Add<V3> for V3 {
    type Output = V3;

    fn add(self, rhs: V3) -> V3 {
        V3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<V3> for V3 {
    type Output = V3;

    fn sub(self, rhs: V3) -> V3 { 
        V3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<V3> for V3 {
    type Output = V3;

    fn mul(self, rhs: V3) -> V3 { 
        V3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f32> for V3 {
    type Output = V3;

    fn mul(self, rhs: f32) -> V3 { 
        V3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    } 
}

impl ops::Div<f32> for V3 {
    type Output = V3;

    fn div(self, rhs: f32) -> V3 { 
        V3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    } 
}

impl V3 {
    fn dot(&self, rhs: &V3) -> f32 { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z } 
    fn norm(&self) -> f32 { self.dot(self).sqrt() }
    fn unit(self) -> V3 { self / self.norm() }
    fn cross(&self, rhs: &V3) -> V3 {
        V3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
    fn rotate(self, rhs: &V3, rads: f32) -> V3 {
        //awesome explanation:
        //https://math.stackexchange.com/a/1432182
        /*
        let par = self * (self.dot(rhs) / rhs.dot(rhs));
        let ort = self - par;
        let w = rhs.cross(&ort);
        let x1 = rads.cos() / ort.norm();
        let x2 = rads.sin() / w.norm();
        let part = (ort * x1 + w * x2) * ort.norm();
        part + par
        */
        (self * rads.cos()) + 
            (*rhs * ((1.0 - rads.cos())*(rhs.dot(&self)))) +
            (rhs.cross(&self) * rads.sin())
    }

}

fn random_dome<R: Rng>(rng: &mut R, normal: V3) -> V3 {
    rng.gen_iter::<(f32, f32, f32)>()
        .map(|(x, y, z)| (V3 {x: x * 2. - 1., y: y * 2. - 1., z: z * 2. - 1.}).unit())
        .filter(|v| v.dot(&normal) >= 0.)
        .next()
        .unwrap()
}

#[derive(Copy, Clone, Debug)]
struct Ray {
    origin: V3,
    direction: V3,
}

impl Ray {
    fn point(self, t: f32) -> V3 {
        V3 {
            x: self.origin.x + self.direction.x * t,
            y: self.origin.y + self.direction.y * t,
            z: self.origin.z + self.direction.z * t,
        }
    }
}

#[derive(Debug)]
struct Camera {
    eye: V3,
    lt: V3,
    rt: V3,
    lb: V3,
}

impl Camera {
    fn rotate(&mut self, rhs: &V3, rads: f32) {
        self.eye = self.eye.rotate(rhs, rads);
        self.lt = self.lt.rotate(rhs, rads);
        self.rt = self.rt.rotate(rhs, rads);
        self.lb = self.lb.rotate(rhs, rads);
    }
}

#[derive(Debug)]
struct Sphere {
    center: V3,
    radius: f32,
    color: V3,
    is_light: bool,
}

struct Hit {
    dist: f32,
    point: V3,
    normal: V3
}

impl Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let dis = b * b - a * c;

        if dis > 0. {
            let e = dis.sqrt();

            let t = (-b - e) / a;
            if t > 0.007 {
                let pt = ray.point(t);
                let n = (pt - self.center).unit();
                return Some(Hit {
                    dist: t,
                    point: pt,
                    normal: n,
                });
            }

            let t = (-b + e) / a;
            if t > 0.007 {
                let pt = ray.point(t);
                let n = (pt - self.center).unit();
                return Some(Hit {
                    dist: t,
                    point: pt,
                    normal: n,
                });
            }
        }
        None
    }
}

#[derive(Debug)]
struct World {
    spheres: Vec<Sphere>,
    camera: Camera,
}

impl World {
    fn new () -> World {
        World {
            camera: Camera {
                eye: V3 {x: 0., y: 4.5, z: 12.},
                lt: V3 {x: -6., y: 9., z: 7.},
                rt: V3 {x: 6., y: 9., z: 7.},
                lb: V3 {x: -6., y: 0., z: 7.}
            },
            spheres: vec![
                Sphere {
                    center: V3 {x: 0., y: -10002., z: 0.},
                    radius: 9999.,
                    color: V3 {x: 1., y: 1., z: 1.},
                    is_light: false,
                },
                Sphere {
                    center: V3 {x: -10012., y: 0., z: 0.},
                    radius: 9999.,
                    color: V3 {x: 1., y: 0., z: 0.},
                    is_light: false,
                },
                Sphere {
                    center: V3 {x: 10012., y: 0., z: 0.},
                    radius: 9999.,
                    color: V3 {x: 0., y: 1., z: 0.},
                    is_light: false,
                },
                Sphere {
                    center: V3 {x: 0., y: 0., z: -10012.},
                    radius: 9999.,
                    color: V3 {x: 1., y: 1., z: 1.},
                    is_light: false,
                },
                Sphere {
                    center: V3 {x: 0., y: 10012., z: 0.},
                    radius: 9999.,
                    color: V3 {x: 1., y: 1., z: 1.},
                    is_light: true,
                },
                Sphere {
                    center: V3 {x: 0.0, y: 13.0, z: 0.0},
                    radius: 1.0,
                    color: V3 {x: 0.0, y: 0.0, z: 0.0},
                    is_light: false,
                },
                Sphere {
                    center: V3 {x: -5., y: 0., z: 2.},
                    radius: 2.,
                    color: V3 {x: 1., y: 1., z: 0.},
                    is_light: false,
                },
                Sphere {
                    center: V3 {x: 0., y: 5., z: -1.},
                    radius: 4.,
                    color: V3 {x: 1., y: 0., z: 0.},
                    is_light: false,
                },
                Sphere {
                    center: V3 {x: 8., y: 5., z: -1.},
                    radius: 2.,
                    color: V3 {x: 0., y: 0., z: 1.},
                    is_light: false,
                }],
        }
    }

    fn trace<R: Rng>(&self, rng: &mut R, ray: &Ray, depth: u32) -> V3 {
        if depth >= MAXDEPTH {
            return V3 {x: 0., y: 0., z: 0.};
        }
        let closest_hit = self.spheres
            .iter()
            .filter_map(|sphere| sphere.hit(ray).map(|hit| (sphere, hit)))
            .fold(None, |old, (sphere, hit)| match old {
                None => Some((sphere, hit)),
                Some((_, ref old_hit)) if hit.dist < old_hit.dist => Some((sphere, hit)),
                _ => old,
            });
        match closest_hit {
            Some((ref sphere, ref hit)) if !sphere.is_light => {
                let nray = Ray {
                    origin: hit.point,
                    direction: random_dome(rng, hit.normal),
                };
                let ncolor = self.trace(rng, &nray, depth + 1);
                let at = nray.direction.dot(&hit.normal);
                sphere.color * (ncolor * at)
            },
            Some((ref sphere, _)) => sphere.color,
            _ => V3 {x: 0., y: 0., z: 0.}
        }
    }
}

fn render(world: &World) -> Vec<V3> {
    let vdu = (world.camera.rt - world.camera.lt) / WIDTH as f32;
    let vdv = (world.camera.lb - world.camera.lt) / HEIGHT as f32;
    let rnd1 = rand::random();
    let rnd2 = rand::random();
    let rnd3 = rand::random();
    //let rnd4 = rand::random();

    (0..HEIGHT*WIDTH).into_par_iter().map(|pixel| {
        let x = pixel % WIDTH;
        let y = pixel / WIDTH;

        let mut rng = rand::XorShiftRng::from_seed([rnd1, rnd2, rnd3, rand::random()]);

        let color: V3 = (0..SAMPLES).map(|_| {
            let ray = Ray {
                origin: world.camera.eye,
                direction: ((world.camera.lt +
                             (vdu * (x as f32 + rng.gen::<f32>()) +
                              vdv * (y as f32 + rng.gen::<f32>()))) -
                            world.camera.eye)
                    .unit(),
            };

            world.trace(&mut rng, &ray, 0)
        }).fold(V3 {x:0., y:0., z:0.}, |a, b| a + b);

        color / SAMPLES as f32
    }).collect()
}

fn main() {
    let sdl_context = sdl2::init().expect("sdl2_context");
    let video_subsystem = sdl_context.video().expect("video_subsystem");
    let window = video_subsystem.window("Light", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .expect("window");

    let mut canvas = window.into_canvas().build().expect("canvas");
    let mut event_pump = sdl_context.event_pump().expect("event_pump");
    let mut prev_time = time::precise_time_s();
    let mut curr_time : f64;
    let mut fps : String;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32).unwrap();
    let rect = Rect::new(0, 0, WIDTH as u32, HEIGHT as u32);

    let mut world = World::new();
    let yaxis = V3{x: 0.0, y: 1.0, z: 0.0};
    let mut count = 0.0;
    let mut acc: Vec<V3> = Vec::new();
    acc.resize(WIDTH * HEIGHT, V3{x:0.0, y:0.0, z:0.0});

    'events_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'events_loop
                },
                Event::KeyDown{ keycode: Some(Keycode::Right), ..} => {
                    world.camera.rotate(&yaxis, 0.01);
                    acc = Vec::new();
                    acc.resize(WIDTH * HEIGHT, V3{x:0.0, y:0.0, z:0.0});
                    count = 0.0;
                },
                Event::KeyDown{ keycode: Some(Keycode::Left), ..} => {
                    world.camera.rotate(&yaxis, -0.01);
                    acc = Vec::new();
                    acc.resize(WIDTH * HEIGHT, V3{x:0.0, y:0.0, z:0.0});
                    count = 0.0;
                },
                _ => {}
            }
        }



        let data = render(&world);
        count = count + 1.0;

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let offset = y*pitch + x*3;
                    let pos = y * WIDTH + x;
                    let d = data[pos];
                    acc[pos] = acc[pos] + d;
                    let pixel = acc[pos] / count;

                    buffer[offset] = (pixel.x.min(1.0) * 255.99) as u8; //B
                    buffer[offset + 1] = (pixel.y.min(1.0) * 255.99) as u8; //G
                    buffer[offset + 2] = (pixel.z.min(1.0) * 255.99) as u8; //R
                }
            }
        }).unwrap();

        canvas.copy(&texture, None, Some(rect)).unwrap();
        curr_time = time::precise_time_s();
        fps = format!("{:.*}", 2, 1.0 / (curr_time - prev_time));
        prev_time = curr_time;
        canvas.string(0,0, &fps, Color::RGB(128, 128, 128)).expect("canvas.string");
        canvas.present();

        //rotate the camera
        //world.camera.rotate(&yaxis, 0.01);
    }
}
