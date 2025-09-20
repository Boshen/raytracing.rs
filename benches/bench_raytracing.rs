use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

use raytracing::{
    args::{ArgCamera, Args},
    renderer::Renderer,
    scene::CornellBox,
};

/// Benchmark simple rendering performance.
///
/// # Panics
/// Panics if the Cornell Box scene cannot be created.
pub fn bench_simple(c: &mut Criterion) {
    let args =
        Args { width: 10, height: 10, preview: false, camera: ArgCamera::ThinLens, samples: 4 };
    let scene = CornellBox::new(args.width, args.height, &args).unwrap();
    let renderer = Renderer::new(Box::new(scene), &args);
    c.bench_function("render", |b| b.iter(|| black_box(renderer.render())));
}

criterion_group!(simple, bench_simple);
criterion_main!(simple);
