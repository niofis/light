extern crate sdl2;
extern crate time;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().expect("sdl2_context");
    let video_subsystem = sdl_context.video().expect("video_subsystem");
    let window = video_subsystem.window("Light", 800, 600)
        .position_centered()
        .build()
        .expect("window");

    let mut canvas = window.into_canvas().build().expect("canvas");
    let mut event_pump = sdl_context.event_pump().expect("event_pump");
    let mut prev_time = time::precise_time_s();
    let mut curr_time : f64;
    let mut fps : String;

    'events_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'events_loop
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        curr_time = time::precise_time_s();
        fps = format!("{:.*}", 2, 1.0 / (curr_time - prev_time));
        prev_time = curr_time;
        canvas.string(0,0, &fps, Color::RGB(255, 255, 255)).expect("canvas.string");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
