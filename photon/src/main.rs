use bincode::{config, Encode};
use clap::{value_parser, Arg, ArgAction, Command};
use light::{demos, Accelerator, Algorithm, Camera, Color, Point, RenderMethod, Renderer, Section};
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
    bincode::encode_to_vec(binary_render, config).unwrap()
}

fn main() {
    let matches = Command::new("Photon")
        .version("0.1")
        .author("Enrique <niofis@gmail.com>")
        .about("Renders a scene using the light engine")
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .action(ArgAction::Set)
                .value_parser(value_parser!(u32))
                .help("specify the number of threads to use, defaults to the number of cpus available"),
        )
        .arg(
            Arg::new("accelerator")
                .long("accelerator")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String))
                .default_value("bvh")
                .help("specify the accelerator structure to use from: brute_force and bvh. Defaults to bvh"))
        .arg(
            Arg::new("demo")
                .short('d')
                .long("demo")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String))
                .help("renders one of the demo scenes: simple, cornell, bunny"))
        .arg(
            Arg::new("obj")
                .short('o')
                .long("obj")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String))
                .conflicts_with("demo")
                .help("renders the specified obj file"))
        .arg(
            Arg::new("stats")
                .long("stats")
                .action(ArgAction::Set)
                .value_parser(value_parser!(u32))
                .conflicts_with("threads")
                .help("captures stats and prints them when done rendering. cannot be used with threads"))
        .arg(
            Arg::new("algorithm")
                .short('a')
                .long("algorithm")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String))
                .help("choose the rendering algorithm, options: pathtracing, whitted. Not setting this option defaults to pathtracing"))
        .arg(
            Arg::new("render method")
                .short('r')
                .long("rendermethod")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String))
                .help("select the rendering method: pixels, tiles, scanlines. Not setting this option defaults to tiles.")
        )
        .arg(
            Arg::new("samples count")
                .short('c')
                .long("samples")
                .action(ArgAction::Set)
                .value_parser(value_parser!(u32))
                .help("specify the number of samples per pixel to collect")
        )
        .arg(
            Arg::new("ppm")
                .short('p')
                .long("ppm")
                .action(ArgAction::SetTrue)
                .help("outputs ppm to the stdio")
        )
        .arg(
            Arg::new("savefile")
                .short('s')
                .long("save")
                .help("saves the ppm to disk using the default name structure: YYYYMMDD-HHMM-SAMPLES-TIME.ppm")
        )
        .arg(
            Arg::new("save binary")
                .short('b')
                .long("binary")
                .help("saves the render result as a binary file using the default name structure: YYYMMDD-HHMM-SAMPLES-TIME.brf"))
        .arg(
            Arg::new("ml")
            .short('m')
            .long("machine-learning")
            .help("generates a 1000 samples image plus 1000 single sample images to use for ml; saves to /.ml folder")
        )
        .arg(
            Arg::new("width")
            .short('w')
            .long("width")
            .value_parser(value_parser!(u32))
            .help("outupt image width")
        )
        .arg(
            Arg::new("height")
            .short('h')
            .long("height")
            .value_parser(value_parser!(u32))
            .help("outupt image height")
        )
        .arg(
            Arg::new("json")
            .long("json")
            .help("load scene from the specified json file")
        )
        .get_matches();
    let mut width: u32 = 640;
    let mut height: u32 = 360;

    if let Some(val) = matches.get_one::<u32>("width") {
        width = *val;
    } else {
        eprintln!("invalid width value!");
    }

    if let Some(val) = matches.get_one::<u32>("height") {
        height = *val;
    } else {
        eprintln!("invalid height value!");
    }

    let mut renderer_builder = Renderer::builder();
    let v_offset = 3.0;
    let z_offset = -10.0;
    renderer_builder
        .width(width)
        .height(height)
        .camera(Camera::new(
            Point(0.0, 9.0 / 2.0 + v_offset, -60.0 - z_offset),
            Point(-8.0, 9.0 + v_offset, -50.0 - z_offset),
            Point(-8.0, 0.0 + v_offset, -50.0 - z_offset),
            Point(8.0, 9.0 + v_offset, -50.0 - z_offset),
        ));

    match matches.get_one::<String>("algorithm") {
        Some(val) if val == "whitted" => {
            renderer_builder.algorithm(Algorithm::Whitted);
        }
        _ => {
            renderer_builder.algorithm(Algorithm::PathTracing);
        }
    }

    match matches.get_one::<String>("render method") {
        Some(val) if val == "pixels" => {
            renderer_builder.render_method(RenderMethod::Pixels);
        }
        Some(val) if val == "scanlines" => {
            renderer_builder.render_method(RenderMethod::Scanlines);
        }
        _ => {
            renderer_builder.render_method(RenderMethod::Tiles);
        }
    }

    if matches.contains_id("stats") {
        renderer_builder.use_stats();
    }

    match matches.get_one::<String>("demo") {
        Some(val) if val == "simple" => {
            renderer_builder.world(demos::simple());
        }
        Some(val) if val == "cornell" => {
            renderer_builder.world(demos::cornell());
        }
        Some(val) if val == "shader_bench" => {
            renderer_builder.world(demos::shader_bench());
        }
        _ => {}
    }

    // if let Some(val) = matches.value_of("obj") {
    //     renderer_builder.world(demos::obj(val));
    // }

    if let Some(json_file) = matches.get_one::<String>("json") {
        let json = fs::read_to_string(json_file).unwrap();
        renderer_builder.from_json(&json);
    }

    match matches.get_one::<String>("accelerator") {
        Some(val) if val == "brute_force" => {
            renderer_builder.accelerator(Accelerator::BruteForce);
        }
        Some(val) if val == "bvh" => {
            renderer_builder.accelerator(Accelerator::BoundingVolumeHierarchy);
        }
        _ => {}
    }

    if let Some(val) = matches.get_one::<u32>("threads") {
        renderer_builder.threads(*val);
    } else {
        eprintln!("invalid threads value!");
    }

    if let Some(val) = matches.get_one::<u32>("samples count") {
        renderer_builder.samples(*val);
    } else {
        renderer_builder.samples(10);
    }

    let section = Section::new(0, 0, width, height);

    if matches.contains_id("ml") {
        let demo: String = matches.get_one::<String>("demo").unwrap().to_owned();
        let start = time::Instant::now();
        renderer_builder.samples(1000);
        let mut renderer = renderer_builder.build();
        let pixels = renderer.render(&section);
        let data = format_as_binary(&pixels, width, height);
        fs::write(format!("./ml/{}-target.brf", demo), data).unwrap();
        let elapsed = start.elapsed().as_seconds_f32();
        eprintln!("Target image rendering time: {}s", elapsed);

        let start = time::Instant::now();
        renderer_builder.samples(1);
        let mut renderer = renderer_builder.build();
        for i in 0..100 {
            let pixels = renderer.render(&section);
            let data = format_as_binary(&pixels, width, height);
            fs::write(format!("./ml/{}-training-{}.brf", demo, i), data).unwrap();
        }
        let elapsed = start.elapsed().as_seconds_f32();
        eprintln!("Training images rendering time: {}s", elapsed);

        return;
    }

    let mut renderer = renderer_builder.build();
    let start = time::Instant::now();
    let pixels = renderer.render(&section);
    let elapsed = start.elapsed().as_seconds_f32();
    eprintln!("Rendering time: {}s", elapsed);

    if matches.contains_id("save binary") {
        let now = chrono::offset::Local::now();
        let date = now.format("%Y%m%d-%H%M%S");
        let filename = format!("{}-{}-{}.brf", date, renderer.samples, elapsed.ceil());
        let data = format_as_binary(&pixels, width, height);
        fs::write(&filename, data).unwrap();
        eprintln!("saved file {}", filename);
    } else if matches.contains_id("savefile") {
        let ppm = format_as_ppm(&pixels, width, height);
        let now = chrono::offset::Local::now();
        let date = now.format("%Y%m%d-%H%M%S");
        let filename = format!("{}-{}-{}.ppm", date, renderer.samples, elapsed.ceil());
        fs::write(&filename, ppm).unwrap();
        eprintln!("saved file {}", filename);
    } else if matches.contains_id("ppm") {
        let ppm = format_as_ppm(&pixels, width, height);
        println!("{}", ppm);
    }
}
