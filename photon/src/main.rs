use bincode::{config, Encode};
use clap::{App, Arg};
use ilios::{demos, Accelerator, Algorithm, Camera, Color, Point, RenderMethod, Renderer, Section};
use std::fs;

#[derive(Encode)]
struct BinaryRender {
    width: u32,
    height: u32,
    pixels: Vec<(f32, f32, f32)>,
}

fn gamma_correct(x: u8) -> u8 {
    (255.0 * (x as f32 / 255.0).powf(1.0 / 2.2)).min(255.0) as u8
}

fn format_as_ppm(pixels: &[Color], width: u32, height: u32) -> String {
    let mut output = String::new();
    output.push_str(&format!("P3\n{} {}\n255\n", width, height));
    for Color(red, green, blue) in pixels.iter() {
        let r = if *red > 1.0 {
            255
        } else {
            (red * 255.99) as u8
        };
        let g = if *green > 1.0 {
            255
        } else {
            (green * 255.99) as u8
        };
        let b = if *blue > 1.0 {
            255
        } else {
            (blue * 255.99) as u8
        };

        output.push_str(&format!(
            "{} {} {} ",
            gamma_correct(r),
            gamma_correct(g),
            gamma_correct(b)
        ));
    }
    output
}

fn format_as_binary(pixels: &[Color], width: u32, height: u32) -> Vec<u8> {
    let px = pixels.iter().map(|Color(r, g, b)| (*r, *g, *b)).collect();
    let binary_render = BinaryRender {
        width,
        height,
        pixels: px,
    };
    let config = config::standard();
    bincode::encode_to_vec(&binary_render, config).unwrap()
}

fn main() {
    let matches = App::new("Photon")
        .version("0.1")
        .author("Enrique <niofis@gmail.com>")
        .about("Renders a scene using the light engine")
        .arg(
            Arg::with_name("threads")
                .short('t')
                .long("threads")
                .takes_value(true)
                .multiple(false)
                .help("specify the number of threads to use, defaults to the number of cpus available"),
        )
        .arg(
            Arg::with_name("accelerator")
                .short('a')
                .long("accelerator")
                .takes_value(true)
                .multiple(false)
                .possible_values(&["brute_force", "bvh"])
                .default_value("bvh")
                .help("specify the accelerator structure to use, defaults to bvh"))
        .arg(
            Arg::with_name("demo")
                .short('d')
                .long("demo")
                .takes_value(true)
                .multiple(false)
                .possible_values(&["simple", "cornell", "bunny"])
                .help("renders one of the demo scenes"))
        .arg(
            Arg::with_name("obj")
                .short('o')
                .long("obj")
                .takes_value(true)
                .multiple(false)
                .conflicts_with("demo")
                .help("renders the specified obj file"))
        .arg(
            Arg::with_name("stats")
                .short('t')
                .long("stats")
                .takes_value(false)
                .multiple(false)
                .conflicts_with("threads")
                .help("captures stats and prints them when done rendering. cannot be used with threads"))
        .arg(
            Arg::with_name("algorithm")
                .short('a')
                .long("algorithm")
                .takes_value(true)
                .multiple(false)
                .help("choose the rendering algorithm, options: pathtracing, whitted. Not setting this option defaults to pathtracing"))
        .arg(
            Arg::with_name("render method")
                .short('r')
                .long("rendermethod")
                .takes_value(true)
                .multiple(false)
                .help("select the rendering method: pixels, tiles, scanlines. Not setting this option defaults to tiles.")
        )
        .arg(
            Arg::with_name("samples count")
                .short('c')
                .long("samples")
                .takes_value(true)
                .multiple(false)
                .help("specify the number of samples per pixel to collect")
        )
        .arg(
            Arg::with_name("ppm")
                .short('p')
                .long("ppm")
                .takes_value(false)
                .multiple(false)
                .help("outputs ppm to the stdio")
        )
        .arg(
            Arg::with_name("savefile")
                .short('s')
                .long("save")
                .takes_value(false)
                .multiple(false)
                .help("saves the ppm to disk using the default name structure: YYYYMMDD-HHMM-SAMPLES-TIME.ppm")
        )
        .arg(
            Arg::with_name("save binary")
                .short('b')
                .long("binary")
                .takes_value(false)
                .multiple(false)
                .help("saves the render result as a binary file using the default name structure: YYYMMDD-HHMM-SAMPLES-TIME.brf"))
        .arg(
            Arg::with_name("ml")
            .short('m')
            .long("machine-learning")
            .takes_value(false)
            .multiple(false)
            .help("generates a 1000 samples image plus 1000 single sample images to use for ml; saves to /.ml folder")
        )
        .arg(
            Arg::with_name("width")
            .short('w')
            .long("width")
            .takes_value(true)
            .multiple(false)
            .help("outupt image width")
        )
        .arg(
            Arg::with_name("height")
            .short('h')
            .long("height")
            .takes_value(true)
            .multiple(false)
            .help("outupt image height")
        )
        .get_matches();
    let mut width: u32 = 640;
    let mut height: u32 = 360;

    if let Some(val) = matches.value_of("width") {
        if let Ok(parsed_width) = val.parse() {
            width = parsed_width;
        } else {
            eprintln!("invalid width value!");
        }
    }

    if let Some(val) = matches.value_of("height") {
        if let Ok(parsed_height) = val.parse() {
            height = parsed_height;
        } else {
            eprintln!("invalid height value!");
        }
    }

    let mut renderer = Renderer::build();
    let v_offset = 3.0;
    let z_offset = -10.0;
    renderer.width(width).height(height).camera(Camera::new(
        Point(0.0, 9.0 / 2.0 + v_offset, -60.0 - z_offset),
        Point(-8.0, 9.0 + v_offset, -50.0 - z_offset),
        Point(-8.0, 0.0 + v_offset, -50.0 - z_offset),
        Point(8.0, 9.0 + v_offset, -50.0 - z_offset),
    ));

    match matches.value_of("algorithm") {
        Some("whitted") => {
            renderer.algorithm(Algorithm::Whitted);
        }
        _ => {
            renderer.algorithm(Algorithm::PathTracing);
        }
    }

    match matches.value_of("render method") {
        Some("pixels") => {
            renderer.render_method(RenderMethod::Pixels);
        }
        Some("scanlines") => {
            renderer.render_method(RenderMethod::Scanlines);
        }
        _ => {
            renderer.render_method(RenderMethod::Tiles);
        }
    }

    if matches.is_present("stats") {
        renderer.use_stats();
    }

    match matches.value_of("demo") {
        Some("simple") => {
            renderer.world(demos::simple());
        }
        Some("cornell") => {
            renderer.world(demos::cornell());
        }
        Some("shader_bench") => {
            renderer.world(demos::shader_bench());
        }
        _ => return println!("scene not found!"),
    }

    if let Some(val) = matches.value_of("obj") {
        renderer.world(demos::obj(val));
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

    let section = Section::new(0, 0, width, height);
    renderer.finish();

    if matches.is_present("ml") {
        let demo = matches.value_of("demo").unwrap();
        let start = time::Instant::now();
        renderer.samples(1000);
        let pixels = renderer.render(&section);
        let data = format_as_binary(&pixels, width, height);
        fs::write(format!("./ml/{}-target.brf", demo), data).unwrap();
        let elapsed = start.elapsed().as_seconds_f32();
        eprintln!("Target image rendering time: {}s", elapsed);

        let start = time::Instant::now();
        renderer.samples(1);
        for i in 0..100 {
            let pixels = renderer.render(&section);
            let data = format_as_binary(&pixels, width, height);
            fs::write(format!("./ml/{}-training-{}.brf", demo, i), data).unwrap();
        }
        let elapsed = start.elapsed().as_seconds_f32();
        eprintln!("Training images rendering time: {}s", elapsed);

        return;
    }

    let start = time::Instant::now();
    let pixels = renderer.render(&section);
    let elapsed = start.elapsed().as_seconds_f32();
    eprintln!("Rendering time: {}s", elapsed);

    if matches.is_present("save binary") {
        let now = chrono::offset::Local::now();
        let date = now.format("%Y%m%d-%H%M%S");
        let filename = format!("{}-{}-{}.brf", date, renderer.samples, elapsed.ceil());
        let data = format_as_binary(&pixels, width, height);
        fs::write(&filename, data).unwrap();
        eprintln!("saved file {}", filename);
    } else if matches.is_present("savefile") {
        let ppm = format_as_ppm(&pixels, width, height);
        let now = chrono::offset::Local::now();
        let date = now.format("%Y%m%d-%H%M%S");
        let filename = format!("{}-{}-{}.ppm", date, renderer.samples, elapsed.ceil());
        fs::write(&filename, ppm).unwrap();
        eprintln!("saved file {}", filename);
    } else if matches.is_present("ppm") {
        let ppm = format_as_ppm(&pixels, width, height);
        println!("{}", ppm);
    }
}
