use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracing::{
    args::{ArgCamera, Args},
    renderer::Renderer,
    scene::CornellBox,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let args =
        Args { width: 10, height: 10, preview: false, camera: ArgCamera::ThinLens, samples: 4 };
    let scene = CornellBox::new(args.width, args.height, &args);
    let renderer = Renderer::new(scene, &args);
    c.bench_function("render", |b| b.iter(|| black_box(renderer.render())));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1000).measurement_time(Duration::new(5, 0));
    targets = criterion_benchmark
}
criterion_main!(benches);
