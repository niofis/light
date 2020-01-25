#[macro_use]
extern crate criterion;

use criterion::Criterion;

use light::light::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Demo 800x450", |b| {
        b.iter(|| {
            let width: u32 = 800;
            let height: u32 = 450;
            let bpp = 4;
            let mut world = World::demo(bpp, width, height);
            world.render();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
