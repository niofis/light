use clap::{App, AppSettings, Arg};
use light::{Accelerator, Color};
use light::{Camera, Vector};

fn print_ppm(data: &[u8], width: usize, height: usize) {
    println!("P3\n{} {}\n255", width, height);
    for pixel in (0..(width * height * 4)).step_by(4) {
        print!("{} {} {} ", data[pixel], data[pixel + 1], data[pixel + 2]);
        print!("\n");
    }
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
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
    let width = 640;
    let height = 480;

    let mut renderer = light::Renderer::build();
    renderer
        .width(width)
        .height(height)
        .camera(Camera::new(
            Vector(0.0, 15.0 / 2.0, -75.0),
            Vector(-20.0 / 2.0, 15.0, -50.0),
            Vector(-20.0 / 2.0, 0.0, -50.0),
            Vector(20.0 / 2.0, 15.0, -50.0),
        ))
        // .render_method(light::RenderMethod::Pixels);
        .render_method(light::RenderMethod::Tiles);

    if let Some(scene) = matches.value_of("demo") {
        match scene {
            "simple" => {
                renderer.world(light::demos::simple());
            }
            "cornell" => {
                renderer.world(light::demos::cornell());
            }
            "shader_bench" => {
                renderer.world(light::demos::shader_bench());
            }
            _ => return println!("scene not found!"),
        }
    }

    if let Some(val) = matches.value_of("obj") {
        renderer.world(light::demos::obj(val));
    }

    if let Some(val) = matches.value_of("accelerator") {
        match val {
            "brute_force" => {
                renderer.accelerator(Accelerator::BruteForce);
            }
            "bvh" => {
                renderer.accelerator(Accelerator::BoundingVolumeHierarchy);
            }
            _ => {}
        }
    }

    if let Some(val) = matches.value_of("threads") {
        if let Ok(threads) = val.parse() {
            renderer.threads(threads);
        } else {
            eprintln!("invalid threads value!");
        }
    }

    let section = light::Section::new(0, 0, width, height);
    renderer.finish();

    let mut buffer: Vec<u8> = vec![0; (4 * width * height) as usize];
    let start = time::precise_time_s();
    let pixels = renderer.render(&section);
    for (idx, pixel) in pixels.into_iter().enumerate() {
        let x = section.x + (idx % section.width);
        let y = section.y + (idx / section.width);
        let offset = (y * width + x) * 4;
        let Color(r, g, b) = pixel;
        buffer[offset] = if r > 1.0 { 255 } else { (r * 255.99) as u8 };
        buffer[offset + 1] = if g > 1.0 { 255 } else { (g * 255.99) as u8 };
        buffer[offset + 2] = if b > 1.0 { 255 } else { (b * 255.99) as u8 };
    }

    let elapsed = time::precise_time_s() - start;
    eprintln!("Rendering time: {}s", elapsed);
    print_ppm(&buffer, width, height);
}
