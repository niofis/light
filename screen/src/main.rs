use light::{Accelerator, Camera, Color, Point, Renderer, World};
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::error::Error;
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn Error>> {
    let width: u32 = 640;
    let height: u32 = 480;
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

    // let mut world = World::bunny(width, height);
    //let mut world = World::shader_bench(width, height);
    let mut renderer = Renderer::build();
    renderer
        .width(width as usize)
        .height(height as usize)
        .camera(Camera::new(
            Point(0.0, 15.0 / 2.0, -75.0),
            Point(-20.0 / 2.0, 15.0, -50.0),
            Point(-20.0 / 2.0, 0.0, -50.0),
            Point(20.0 / 2.0, 15.0, -50.0),
        ))
        .render_method(light::RenderMethod::Tiles)
        .world(light::demos::cornell())
        .accelerator(Accelerator::BoundingVolumeHierarchy)
        .finish();

    let mut buffer: Vec<u8> = vec![0; (4 * width * height) as usize];
    let section = light::Section::new(0, 0, width as usize, height as usize);

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

        let pixels = renderer.render(&section);
        for (idx, pixel) in pixels.into_iter().enumerate() {
            let x = section.x + (idx % section.width);
            let y = section.y + (idx / section.width);
            let offset = (y * (width as usize) + x) * 4;
            let Color(red, green, blue) = pixel;
            buffer[offset + 2] = if red > 1.0 { 255 } else { (red * 255.99) as u8 };
            buffer[offset + 1] = if green > 1.0 {
                255
            } else {
                (green * 255.99) as u8
            };
            buffer[offset] = if blue > 1.0 {
                255
            } else {
                (blue * 255.99) as u8
            };
        }

        texture.update(rect, &buffer, step)?;

        canvas.copy(&texture, None, Some(rect))?;

        curr_time = time::precise_time_s();
        fps = format!("{:.*}", 2, 1.0 / (curr_time - prev_time));
        prev_time = curr_time;
        canvas.string(0, 0, &fps, sdl2::pixels::Color::RGB(127, 127, 127))?;
        canvas.present();
    }
    Ok(())
}
