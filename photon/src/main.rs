use clap::{App, AppSettings, Arg, SubCommand};
use light::Accelerator;
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
        .subcommand(
            SubCommand::with_name("demo")
                .about("renders one of the demo scenes: simple, cornell, bunny")
                .arg(
                    Arg::with_name("scene")
                        .required(true)
                        .help("one of the following: simple, cornell, bunny")
                        .takes_value(true),
                ),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("demo") {
        if let Some(scene) = matches.value_of("scene") {
            let mut renderer = light::Renderer::build();
            renderer.width(640).height(480).camera(Camera::new(
                Vector(0.0, 15.0 / 2.0, -75.0),
                Vector(-20.0 / 2.0, 15.0, -50.0),
                Vector(-20.0 / 2.0, 0.0, -50.0),
                Vector(20.0 / 2.0, 15.0, -50.0),
            ));
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
            renderer
                .accelerator(Accelerator::BoundingVolumeHierarchy)
                .finish();
            let buffer = renderer.render();
            print_ppm(&buffer, 640, 480);
        }
    }
}
