use clap::{App, Arg};
use light::{Accelerator, Section};
use light::{Camera, Point};

fn main() {
    let _matches = App::new("Bench")
        .version("0.1")
        .author("Enrique <niofis@gmail.com>")
        .about("Benchmarks the light engine")
        .arg(
            Arg::with_name("no save")
                .short("ns")
                .long("no-save")
                .takes_value(false)
                .multiple(false)
                .help("don't save the result"),
        )
        .arg(
            Arg::with_name("compare")
                .short("c")
                .long("compare")
                .takes_value(false)
                .multiple(false)
                .help("compares agains the latest saved run"),
        )
        .arg(
            Arg::with_name("comment")
                .short("C")
                .long("comment")
                .takes_value(true)
                .multiple(false)
                .help("adds a comment to the saved result"),
        )
        .get_matches();

    let width = 1024; //3200;
    let height = 768; //2400;
    let threads = num_cpus::get();

    let mut renderer = light::Renderer::build();
    renderer
        .width(width)
        .height(height)
        .camera(Camera::new(
            Point(0.0, 15.0 / 2.0, -75.0),
            Point(-20.0 / 2.0, 15.0, -50.0),
            Point(-20.0 / 2.0, 0.0, -50.0),
            Point(20.0 / 2.0, 15.0, -50.0),
        ))
        .world(light::demos::cornell())
        .accelerator(Accelerator::BoundingVolumeHierarchy)
        .render_method(light::RenderMethod::Tiles)
        // .render_method(light::RenderMethod::Pixels)
        .threads(1)
        .finish();

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum: f64 = 0.;
    let iterations = 10;
    let section = Section::new(0, 0, width, height);
    for _i in 0..iterations {
        let start = time::precise_time_s();
        let _buffer = renderer.render(&section);
        let elapsed = time::precise_time_s() - start;
        min = min.min(elapsed);
        max = max.max(elapsed);
        sum = sum + elapsed;
    }
    let avg = sum / (iterations as f64);
    println!(
        "Bench results:\nmin:\t{}s\nmax:\t{}s\navg:\t{}s\ntotal:\t{}s\nthreads:\t{}",
        min, max, avg, sum, threads
    );
}
