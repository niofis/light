use light::float::PI;
use light::{
    demos, parsers, Accelerator, Algorithm, BvhBuildMethod, Camera, Color, Point, RenderMethod,
    Renderer, Section, Transform,
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::error::Error;
use std::fs;
use time::Instant;

struct FrameTimmings {
    pub count: u32,
    pub min: f32,
    pub max: f32,
    pub avg: f32,
    pub acc: f32,
}

impl FrameTimmings {
    pub fn new() -> FrameTimmings {
        FrameTimmings {
            count: 0,
            min: f32::MAX,
            max: f32::MIN,
            avg: 0.0,
            acc: 0.0,
        }
    }
    pub fn add(&mut self, elapsed: f32) {
        self.count += 1;
        self.acc += elapsed;
        self.min = self.min.min(elapsed);
        self.max = self.max.max(elapsed);
        self.avg = self.acc / self.count as f32;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let width: u32 = 640;
    let height: u32 = 480; //360;
    let scene: &str = "../photon/torus.json";
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

    let mut fps: String;

    let mut renderer_builder = Renderer::builder();
    renderer_builder
        .width(width)
        .height(height)
        .camera(Camera::new(
            Point(0.0, 0.75, -36.0),
            Point(-1.0, 1.5, -35.0),
            Point(-1.0, 0.0, -35.0),
            Point(1.0, 1.5, -35.0),
        ))
        .algorithm(Algorithm::PathTracing)
        .render_method(RenderMethod::Tiles)
        .world(demos::cornell())
        // .from_json(&fs::read_to_string("../photon/scene.json")?)
        .threads(1)
        .bvh_build_method(BvhBuildMethod::Sah)
        .accelerator(Accelerator::BoundingVolumeHierarchy);

    let mut frames: Vec<Color> = vec![Color(0., 0., 0.); (4 * width * height) as usize];
    let mut buffer: Vec<u8> = vec![0; (4 * width * height) as usize];
    let section = Section::new(0, 0, width, height);
    let mut frames_count: f32 = 0.0;

    let mut frame_timmings = FrameTimmings::new();
    let mut reset: bool = false;

    // let cs1 = Vector::new(0.0, 1.0, 0.0).coordinate_system();
    // let u = Vector::new(1.0, 2.0, 3.0).unit();
    // let (v, w) = u.coordinate_system();

    // println!(
    //     "Coordinate system: {:?} dot:{}\n{:?} dot:{:?}",
    //     cs1,
    //     cs1.0.dot(&cs1.1),
    //     (u, v, w),
    //     (u.dot(&v), u.dot(&w), v.dot(&w))
    // );

    let mut modified = fs::metadata(scene)?.modified()?;

    let res = parsers::json(&fs::read_to_string(scene)?);
    renderer_builder.camera(res.0).world(res.1);

    let mut renderer = renderer_builder.build();

    // let json = fs::read_to_string("../photon/scene.json")?;
    // renderer_builder.from_json(&json);

    'event_loop: loop {
        let timer = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'event_loop;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Left {
                        renderer_builder.camera.apply_transform(&Transform::rotate(
                            0.0,
                            -PI / 100.0,
                            0.0,
                        ));
                    } else if keycode == Keycode::Right {
                        renderer_builder.camera.apply_transform(&Transform::rotate(
                            0.0,
                            PI / 100.0,
                            0.0,
                        ));
                    } else if keycode == Keycode::Up {
                        renderer_builder.camera.apply_transform(&Transform::rotate(
                            -PI / 100.0,
                            0.0,
                            0.0,
                        ));
                    } else if keycode == Keycode::Down {
                        renderer_builder.camera.apply_transform(&Transform::rotate(
                            PI / 100.0,
                            0.0,
                            0.0,
                        ));
                    } else if keycode == Keycode::W {
                        renderer_builder
                            .camera
                            .apply_transform(&Transform::translate(0.0, 0.0, 5.0));
                    } else if keycode == Keycode::S {
                        renderer_builder
                            .camera
                            .apply_transform(&Transform::translate(0.0, 0.0, -5.0));
                    } else if keycode == Keycode::A {
                        renderer_builder
                            .camera
                            .apply_transform(&Transform::translate(5.0, 0.0, 0.0));
                    } else if keycode == Keycode::D {
                        renderer_builder
                            .camera
                            .apply_transform(&Transform::translate(-5.0, 0.0, 0.0));
                    } else if keycode == Keycode::Q {
                        renderer_builder
                            .camera
                            .apply_transform(&Transform::translate(0.0, 5.0, 0.0));
                    } else if keycode == Keycode::E {
                        renderer_builder
                            .camera
                            .apply_transform(&Transform::translate(0.0, -5.0, 0.0));
                    }

                    reset = true;
                }
                _ => {}
            }
        }

        let new_modified = fs::metadata(scene)?.modified()?;
        if modified != new_modified {
            modified = new_modified;
            let res = parsers::json(&fs::read_to_string(scene)?);
            renderer_builder.world(res.1);
            renderer = renderer_builder.build();
            reset = true;
        }

        if reset {
            frames = vec![Color(0., 0., 0.); (4 * width * height) as usize];
            buffer = vec![0; (4 * width * height) as usize];
            frames_count = 0.0;
            frame_timmings = FrameTimmings::new();
            reset = false;
        }

        let pixels = renderer.render(&section);
        frames_count += 1.0;
        for (idx, pixel) in pixels.into_iter().enumerate() {
            let point = frames[idx] + pixel;
            let x = section.left + (idx as u32 % section.width);
            let y = section.top + (idx as u32 / section.width);
            let offset = (y * (width) + x) * 4;
            let (red, green, blue) = (point / frames_count.into()).as_gamma_corrected_rgb_u8();
            buffer[offset as usize] = blue;
            buffer[(offset + 1) as usize] = green;
            buffer[(offset + 2) as usize] = red;
            frames[idx] = point;
        }

        texture.update(rect, &buffer, step)?;

        canvas.copy(&texture, None, Some(rect))?;

        let elapsed = timer.elapsed().as_seconds_f32();
        frame_timmings.add(elapsed);
        fps = format!(
            "fps: {:.*} | min: {:.*}s | max: {:.*}s | avg: {:.*}s | n: {}",
            2,
            1.0 / elapsed,
            4,
            frame_timmings.min,
            4,
            frame_timmings.max,
            4,
            frame_timmings.avg,
            frame_timmings.count
        );

        canvas.present();
        canvas.window_mut().set_title(&fps).unwrap();
    }
    Ok(())
}
