use bincode::{Encode, config};
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use ilios::{Accelerator, Algorithm, BvhBuildMethod, RenderMethod, Renderer, demos};
use ilios_types::camera::Camera;
use ilios_types::color::Color;
use ilios_types::geometry::Point;
use ilios_types::section::Section;
use kosmos::SceneDescriptor;
use std::io::{self, Write};
use std::{fs, io::BufWriter};

const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 360;
const DEFAULT_THREADS: u32 = 0;
const DEFAULT_SAMPLES: u32 = 10;

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

fn format_as_png(pixels: &[Color], width: u32, height: u32) -> Vec<u8> {
    let mut file = Vec::new();
    {
        let w = BufWriter::new(&mut file);
        let mut encoder = png::Encoder::new(w, width, height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let mut data: Vec<u8> = vec![];
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
            data.push(gamma_correct(r));
            data.push(gamma_correct(g));
            data.push(gamma_correct(b));
        }
        writer.write_image_data(&data).unwrap();
    }
    file
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

fn process_cli() -> ArgMatches {
    Command::new("Photon")
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
            Arg::new("ply")
                .long("ply")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String))
                .conflicts_with("demo")
                .help("renders the specified ply file"))
        .arg(
            Arg::new("scene")
                .short('s')
                .long("scene")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String))
                .conflicts_with("ply")
                .help("render the specified scene, should point to a folder")
        )
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
                .long("ppm")
                .action(ArgAction::SetTrue)
                .help("outputs ppm to the stdio")
        )
        .arg(
            Arg::new("png")
                .long("png")
                .action(ArgAction::SetTrue)
                .help("requires --save")
        )
        .arg(
            Arg::new("savefile")
                .short('s')
                .long("save")
                .action(ArgAction::SetTrue)
                .help("saves the image file to disk using the default name structure: YYYYMMDD-HHMM-SAMPLES-TIME.ppm")
        )
        .arg(
            Arg::new("save binary")
                .short('b')
                .long("binary")
                .action(ArgAction::SetTrue)
                .help("saves the render result as a binary file using the default name structure: YYYMMDD-HHMM-SAMPLES-TIME.brf"))
        .arg(
            Arg::new("ml")
            .short('m')
            .long("machine-learning")
            .help("generates a 1000 samples image plus 1000 single sample images to use for ml; saves to /.ml folder")
        )
        .arg(
            Arg::new("width")
            .long("width")
            .value_parser(value_parser!(u32))
            .help("outupt image width")
        )
        .arg(
            Arg::new("height")
            .long("height")
            .value_parser(value_parser!(u32))
            .help("outupt image height")
        )
        .arg(
            Arg::new("json")
            .long("json")
            .help("load scene from the specified json file")
        )
        .arg(
            Arg::new("BVH build method")
                .long("bvh-build-method")
                .help("Selects the build method to use for building th BVH. Options: octree, sah")
        )
        .get_matches()
}

fn build_renderer(matches: &ArgMatches) -> Renderer {
    let v_offset = 3.0;
    let z_offset = -10.0;
    let mut renderer_builder = Renderer::builder();

    renderer_builder
        .width(
            matches
                .get_one::<u32>("width")
                .map_or(DEFAULT_WIDTH, |v| *v),
        )
        .height(
            matches
                .get_one::<u32>("height")
                .map_or(DEFAULT_HEIGHT, |v| *v),
        )
        .camera(Camera::new(
            Point(0.0, 9.0 / 2.0 + v_offset, -60.0 - z_offset),
            Point(-8.0, 9.0 + v_offset, -50.0 - z_offset),
            Point(-8.0, 0.0 + v_offset, -50.0 - z_offset),
            Point(8.0, 9.0 + v_offset, -50.0 - z_offset),
        ))
        .algorithm(match matches.get_one::<String>("algorithm") {
            Some(val) if val == "whitted" => Algorithm::Whitted,
            _ => Algorithm::PathTracing,
        })
        .render_method(match matches.get_one::<String>("render method") {
            Some(val) if val == "pixels" => RenderMethod::Pixels,
            Some(val) if val == "scanlines" => RenderMethod::Scanlines,
            _ => RenderMethod::Tiles,
        })
        .threads(
            matches
                .get_one::<u32>("threads")
                .map_or(DEFAULT_THREADS, |v| *v),
        )
        .samples(
            matches
                .get_one::<u32>("samples count")
                .map_or(DEFAULT_SAMPLES, |v| *v),
        )
        .accelerator(match matches.get_one::<String>("accelerator") {
            Some(val) if val == "brute-force" => Accelerator::BruteForce,
            Some(val) if val == "bvh" => Accelerator::BoundingVolumeHierarchy,
            _ => Accelerator::BoundingVolumeHierarchy,
        })
        .bvh_build_method(match matches.get_one::<String>("BVH build method") {
            Some(val) if val == "octree" => BvhBuildMethod::Octree,
            _ => BvhBuildMethod::Sah,
        });

    match matches.get_one::<String>("demo") {
        Some(val) if val == "simple" => {
            renderer_builder.world(demos::simple());
        }
        Some(val) if val == "cornell" => {
            renderer_builder.world(demos::cornell());
        }
        _ => {}
    }

    if let Some(scene_path) = matches.get_one::<String>("scene") {
        let SceneDescriptor { camera, world } = kosmos::load(scene_path).unwrap();
        renderer_builder.camera(camera);
        renderer_builder.world(world);
    }

    renderer_builder.build()
}

fn output_image(
    matches: &ArgMatches,
    samples: u32,
    elapsed: f32,
    width: u32,
    height: u32,
    pixels: &[Color],
) {
    if matches.get_flag("save binary") {
        let now = chrono::offset::Local::now();
        let date = now.format("%Y%m%d-%H%M%S");
        let filename = format!("renders/{}-{}-{}.brf", date, samples, elapsed.floor());
        let data = format_as_binary(pixels, width, height);
        fs::write(&filename, data).unwrap();
        eprintln!("saved file {}", filename);
    } else if matches.get_flag("savefile") {
        let (extension, data) = if matches.get_flag("png") {
            ("png", format_as_png(pixels, width, height))
        } else {
            ("ppm", format_as_ppm(pixels, width, height).into())
        };
        let now = chrono::offset::Local::now();
        let date = now.format("%Y%m%d-%H%M%S");
        let filename = format!(
            "renders/{}-{}-{}.{}",
            date,
            samples,
            elapsed.floor(),
            extension
        );
        fs::write(&filename, data).unwrap();
        eprintln!("saved file {}", filename);
    } else if matches.get_flag("ppm") {
        let ppm = format_as_ppm(pixels, width, height);
        println!("{}", ppm);
    } else if matches.get_flag("png") {
        let png = format_as_png(pixels, width, height);
        io::stdout().write_all(&png).unwrap();
    }
}

fn main() {
    let matches = process_cli();
    let mut renderer = build_renderer(&matches);
    let width: u32 = renderer.width;
    let height: u32 = renderer.height;

    let section = Section::new(0, 0, width, height);
    let start = time::Instant::now();
    let pixels = renderer.render(&section);
    let elapsed = start.elapsed().as_seconds_f32() * 1000.0;
    eprintln!("Rendering time: {}ms", elapsed);

    output_image(&matches, renderer.samples, elapsed, width, height, &pixels);
}
