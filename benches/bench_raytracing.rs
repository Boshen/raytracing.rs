use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use raytracing::{
    args::{ArgCamera, Args},
    renderer::Renderer,
    scene::CornellBox,
};

pub fn bench_simple(c: &mut Criterion) {
    c.bench_function("render", |b| {
        b.iter(|| {
            let args = Args {
                width: 100,
                height: 100,
                preview: false,
                camera: ArgCamera::ThinLens,
                samples: 16,
            };
            let scene = CornellBox::new(args.width, args.height, &args);
            let renderer = Renderer::new(scene, &args);
            black_box(renderer.render());
        });
    });
}

criterion_group!(simple, bench_simple);
criterion_main!(simple);
