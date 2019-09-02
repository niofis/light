extern crate sdl2;
extern crate time;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::error::Error;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

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
    let rect = Rect::new(0, 0, WIDTH as u32, HEIGHT as u32);

    let mut prev_time = time::precise_time_s();
    let mut curr_time: f64;
    let mut fps: String;

    let data: Vec<u8> = vec![128; 4 * WIDTH * HEIGHT];

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
        texture.update(rect, &data, 4 * WIDTH)?;

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
        canvas.string(0, 0, &fps, Color::RGB(255, 255, 255))?;
        canvas.present();
    }
    Ok(())
}
