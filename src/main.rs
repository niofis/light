extern crate sdl2;
extern crate time;
extern crate light;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
// use std::time::Duration;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use light::render;
use light::V3;
use light::World;

const WIDTH: usize = 427;
const HEIGHT: usize = 240;


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

    let yaxis = V3{x: 0.0, y: 1.0, z: 0.0};
    let mut world = World::new();
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
