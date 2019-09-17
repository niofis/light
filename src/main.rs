extern crate sdl2;
extern crate time;
use rayon::prelude::*;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::error::Error;
use std::ops;

const WIDTH: usize = 800;
const HEIGHT: usize = 450;

struct Color(f32, f32, f32);
struct Vector(f32, f32, f32);
struct Ray(Vector, Vector);
struct Camera {
    eye: Vector,
    left_top: Vector,
    left_bottom: Vector,
    right_top: Vector,
    width: f32,
    height: f32,
}
enum Primitive {
    Sphere(Vector, f32),
    Triangle(Vector, Vector, Vector, Vector),
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 + x1, y0 + y1, z0 + z1)
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        let Vector(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}

impl Camera {
    fn new(eye: Vector, left_top: Vector, width: f32, height: f32) -> Camera {
        let Vector(x0, y0, z0) = left_top;
        let left_bottom = Vector(x0, y0 - height, z0);
        let right_top = Vector(x0 + width, y0, z0);

        Camera {
            eye,
            left_top,
            left_bottom,
            right_top,
            width,
            height,
        }
    }
}

/*fn render() -> Vec<u8> {
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
fn render2() -> Vec<u8> {
    let bpp = 4;
    let pixels = (0..HEIGHT * WIDTH).map(|pixel| {
        //let x = pixel % WIDTH;
        //let y = pixel / WIDTH;

        ColorF(1.0, 0.0, 0.0)
    });

    let mut buffer = Vec::new();
    let buffer: Vec<u8> = pixels.fold(buffer, |mut acc, pixel| {
        let ColorF(r, g, b) = pixel;
        acc.push((b * 255.99) as u8);
        acc.push((g * 255.99) as u8);
        acc.push((r * 255.99) as u8);
        acc.push(255 as u8);
        acc
    });
    buffer
}

fn render3() -> Vec<u8> {
    let bpp = 4;
    let pixels: Vec<ColorF> = (0..HEIGHT * WIDTH)
        .map(|pixel| {
            //let x = pixel % WIDTH;
            //let y = pixel / WIDTH;

            ColorF(1.0, 0.0, 0.0)
        })
        .collect();
    let mut buffer: Vec<u8> = vec![0; bpp * WIDTH * HEIGHT];
    for pixel in 0..HEIGHT * WIDTH {
        let ColorF(r, g, b) = pixels[pixel];
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

fn render() -> Vec<u8> {
    let bpp = 4;
    let pixels = (0..HEIGHT * WIDTH).map(|pixel| Color(1.0, 0.0, 0.0));

    let mut buffer: Vec<u8> = vec![0; bpp * WIDTH * HEIGHT];
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

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Light v2", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut canvas = window.into_canvas().accelerated().build()?;
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::ARGB8888,
        WIDTH as u32,
        HEIGHT as u32,
    )?;
    let bpp = 4;
    let rect = Rect::new(0, 0, WIDTH as u32, HEIGHT as u32);

    let mut prev_time = time::precise_time_s();
    let mut curr_time: f64;
    let mut fps: String;

    //let data: Vec<u8> = vec![128; 4 * WIDTH * HEIGHT];
    //let mut buffer: Vec<u8> = vec![0; bpp * WIDTH * HEIGHT];

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'event_loop;
                }
                _ => {}
            }
        }

        //canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas.clear();

        /*
                texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    for y in 0..HEIGHT {
                        for x in 0..WIDTH {
                            let offset = y * pitch + x * 3;
                            buffer[offset] = 0 as u8; //R
                            buffer[offset + 1] = 0 as u8; //G
                            buffer[offset + 2] = 255 as u8; //B
                        }
                    }
                })?;
        */
        let buffer = render();
        texture.update(rect, &buffer, bpp * WIDTH)?;

        canvas.copy(&texture, None, Some(rect))?;

        /*canvas.with_texture_canvas(&mut texture, |texture_canvas| {
                    texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
                    texture_canvas.clear();
                    texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
                    texture_canvas.fill_rect(Rect::new(50, 50, 50, 50)).unwrap();
                });
        */
        curr_time = time::precise_time_s();
        fps = format!("{:.*}", 2, 1.0 / (curr_time - prev_time));
        prev_time = curr_time;
        canvas.string(0, 0, &fps, sdl2::pixels::Color::RGB(255, 255, 255))?;
        canvas.present();
    }
    Ok(())
}
