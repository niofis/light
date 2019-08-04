extern crate time;
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let sdl_context = sdl2::init().expect("init");
    let video_subsystem = sdl_context.video().expect("video");
    let window = video_subsystem.window("Light v2", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .expect("window");
    let mut event_pump = sdl_context.event_pump().expect("event_pump");
    let mut canvas = window.into_canvas().build().expect("into_canvas");

    let mut prev_time = time::precise_time_s();
    let mut curr_time: f64 = 0.0;
    let mut fps: String;

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'event_loop;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        curr_time = time::precise_time_s();
        fps = format!("{:.*}", 2, 1.0 / (curr_time - prev_time));
        prev_time = curr_time;
        canvas.string(0, 0, &fps, Color::RGB(255,255,255)).expect("canvas_string");
        canvas.present();
    }
}
