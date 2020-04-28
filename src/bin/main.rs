use light::light::*;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::error::Error;
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn Error>> {
    let width: u32 = 320;
    let height: u32 = 240;
    let bpp = 4;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Light v2", width, height)
        //.fullscreen()
        .position_centered()
        .build()?;
    sdl_context.mouse().show_cursor(false);
    let mut event_pump = sdl_context.event_pump()?;
    let mut canvas = window.into_canvas().accelerated().build()?;
    let texture_creator = canvas.texture_creator();
    let mut texture =
        texture_creator.create_texture_streaming(PixelFormatEnum::ARGB8888, width, height)?;
    let rect = Rect::new(0, 0, width, height);
    let step = (bpp * width) as usize;

    let mut prev_time = time::precise_time_s();
    let mut curr_time: f64;
    let mut fps: String;

    let mut world = World::bunny(width, height);
    //let mut world = World::shader_bench(width, height);
    //let mut world = World::demo(width, height);

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

        world.rotate_camera(PI / 100.0);
        world.rotate_light(PI / 100.0);
        let buffer = world.render();
        texture.update(rect, &buffer, step)?;

        canvas.copy(&texture, None, Some(rect))?;

        curr_time = time::precise_time_s();
        fps = format!("{:.*}", 2, 1.0 / (curr_time - prev_time));
        prev_time = curr_time;
        canvas.string(0, 0, &fps, sdl2::pixels::Color::RGB(255, 255, 255))?;
        canvas.present();
    }
    Ok(())
}
