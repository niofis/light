use clap::{App, AppSettings, Arg};
use light::{Accelerator, Camera, Color, Point, Renderer};
use std::fs;

fn gamma_correct(x: u8) -> u8 {
    (255.0 * (x as f32 / 255.0).powf(1.0 / 2.2)).min(255.0) as u8
}

fn print_ppm(data: &[u8], width: u32, height: u32) -> String {
    let mut output = String::new();
    output.push_str(&format!("P3\n{} {}\n255\n", width, height));
    for pixel in (0..(width * height * 4)).step_by(4) {
        output.push_str(&format!(
            "{} {} {} ",
            gamma_correct(data[pixel as usize]),
            gamma_correct(data[(pixel + 1) as usize]),
            gamma_correct(data[(pixel + 2) as usize])
        ));
    }
    output
}

fn main() {
    let matches = App::new("Photon")
        .version("0.1")
        .author("Enrique <niofis@gmail.com>")
        .about("Renders a scene using the light engine")
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .takes_value(true)
                .multiple(false)
                .help("specify the number of threads to use, defaults to the number of cpus available"),
        )
        .arg(
            Arg::with_name("accelerator")
                .short("a")
                .long("accelerator")
                .takes_value(true)
                .multiple(false)
                .possible_values(&["brute_force", "bvh"])
                .default_value("bvh")
                .help("specify the accelerator structure to use, defaults to bvh"))
        .arg(
            Arg::with_name("demo")
                .short("d")
                .long("demo")
                .takes_value(true)
                .multiple(false)
                .possible_values(&["simple", "cornell", "bunny"])
                .help("renders one of the demo scenes"))
        .arg(
            Arg::with_name("obj")
                .short("o")
                .long("obj")
                .takes_value(true)
                .multiple(false)
                .conflicts_with("demo")
                .help("renders the specified obj file"))
        .arg(
            Arg::with_name("stats")
                .short("s")
                .long("stats")
                .takes_value(false)
                .multiple(false)
                .conflicts_with("threads")
                .help("captures stats and prints them when done rendering. cannot be used with threads"))
        .arg(
            Arg::with_name("algorithm")
                .short("alg")
                .long("algorithm")
                .takes_value(true)
                .multiple(false)
                .help("choose the rendering algorithm, options: pathtracing, whitted. Not setting this option defaults to pathtracing"))
        .arg(
            Arg::with_name("render method")
                .short("rm")
                .long("rendermethod")
                .takes_value(true)
                .multiple(false)
                .help("select the rendering method: pixels, tiles, scanlines. Not setting this option defaults to tiles.")
        )
        .arg(
            Arg::with_name("samples count")
                .short("sc")
                .long("samples")
                .takes_value(true)
                .multiple(false)
                .help("specify the number of samples per pixel to collect")
        )
        .arg(
            Arg::with_name("savefile")
                .short("sv")
                .long("save")
                .takes_value(false)
                .multiple(false)
                .help("saves the ppm to disk using the default name structure: YYYYMMDD-HHMM-SAMPLES-TIME.ppm")
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
    let width: u32 = 1280;
    let height: u32 = 960;

    let mut renderer = Renderer::build();
    renderer.width(width).height(height).camera(Camera::new(
        Point(0.0, 15.0 / 2.0, -75.0),
        Point(-20.0 / 2.0, 15.0, -50.0),
        Point(-20.0 / 2.0, 0.0, -50.0),
        Point(20.0 / 2.0, 15.0, -50.0),
    ));

    match matches.value_of("algorithm") {
        Some("whitted") => {
            renderer.algorithm(light::Algorithm::Whitted);
        }
        _ => {
            renderer.algorithm(light::Algorithm::PathTracing);
        }
    }

    match matches.value_of("render method") {
        Some("pixels") => {
            renderer.render_method(light::RenderMethod::Pixels);
        }
        Some("scanlines") => {
            renderer.render_method(light::RenderMethod::Scanlines);
        }
        _ => {
            renderer.render_method(light::RenderMethod::Tiles);
        }
    }

    if matches.is_present("stats") {
        renderer.use_stats();
    }

    match matches.value_of("demo") {
        Some("simple") => {
            renderer.world(light::demos::simple());
        }
        Some("cornell") => {
            renderer.world(light::demos::cornell());
        }
        Some("shader_bench") => {
            renderer.world(light::demos::shader_bench());
        }
        _ => return println!("scene not found!"),
    }

    if let Some(val) = matches.value_of("obj") {
        renderer.world(light::demos::obj(val));
    }

    match matches.value_of("accelerator") {
        Some("brute_force") => {
            renderer.accelerator(Accelerator::BruteForce);
        }
        Some("bvh") => {
            renderer.accelerator(Accelerator::BoundingVolumeHierarchy);
        }
        _ => {}
    }

    if let Some(val) = matches.value_of("threads") {
        if let Ok(threads) = val.parse() {
            renderer.threads(threads);
        } else {
            eprintln!("invalid threads value!");
        }
    }

    if let Some(val) = matches.value_of("samples count") {
        if let Ok(samples) = val.parse() {
            renderer.samples(samples);
        } else {
            eprintln!("invalid samples value!");
        }
    } else {
        renderer.samples(10);
    }

    let section = light::Section::new(0, 0, width, height);
    renderer.finish();

    let mut buffer: Vec<u8> = vec![0; (4 * width * height) as usize];
    let start = time::precise_time_s();
    let pixels = renderer.render(&section);
    for (idx, pixel) in pixels.into_iter().enumerate() {
        let x = section.x + (idx as u32 % section.width);
        let y = section.y + (idx as u32 / section.width);
        let offset = (y * width + x) * 4;
        let Color(red, green, blue) = pixel;
        buffer[offset as usize] = if red > 1.0 { 255 } else { (red * 255.99) as u8 };
        buffer[(offset + 1) as usize] = if green > 1.0 {
            255
        } else {
            (green * 255.99) as u8
        };
        buffer[(offset + 2) as usize] = if blue > 1.0 {
            255
        } else {
            (blue * 255.99) as u8
        };
    }

    let elapsed = time::precise_time_s() - start;
    eprintln!("Rendering time: {}s", elapsed);
    if let Some(stats) = renderer.stats {
        eprintln!("{:#?}", stats);
    }
    let ppm = print_ppm(&buffer, width, height);
    if matches.is_present("savefile") {
        let now = chrono::offset::Local::now();
        let date = now.format("%Y%m%d-%H%M%S");
        let filename = format!("{}-{}-{}.ppm", date, renderer.samples, elapsed.ceil());
        fs::write(&filename, ppm).unwrap();
        eprintln!("saved file {}", filename);
    } else {
        print!("{}", ppm);
    }
}
