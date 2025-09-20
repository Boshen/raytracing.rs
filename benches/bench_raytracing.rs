use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

use raytracing::{
    args::{ArgCamera, Args},
    renderer::Renderer,
    scene::CornellBox,
};

pub fn bench_simple(c: &mut Criterion) {
    let args =
        Args { width: 10, height: 10, preview: false, camera: ArgCamera::ThinLens, samples: 4 };
    let scene = CornellBox::new(args.width, args.height, &args);
    let renderer = Renderer::new(scene, &args);
    c.bench_function("render", |b| b.iter(|| black_box(renderer.render())));
}

pub fn bench_renderer_100x100(c: &mut Criterion) {
    c.bench_function("renderer_100x100_preview", |b| {
        let args = Args {
            width: 100,
            height: 100,
            preview: true, // Use preview for faster benchmark
            camera: ArgCamera::Simple,
            samples: 1,
        };

        b.iter(|| {
            let scene = CornellBox::new(args.width, args.height, &args);
            let renderer = Renderer::new(scene, &args);
            black_box(renderer.render())
        });
    });
}

pub fn bench_renderer_quality_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("quality_comparison");

    // Preview mode (fast)
    let args_preview =
        Args { width: 50, height: 50, preview: true, camera: ArgCamera::Simple, samples: 1 };

    group.bench_function("preview_50x50", |b| {
        b.iter(|| {
            let scene = CornellBox::new(args_preview.width, args_preview.height, &args_preview);
            let renderer = Renderer::new(scene, &args_preview);
            black_box(renderer.render())
        });
    });

    // Production mode (higher quality)
    let args_production =
        Args { width: 50, height: 50, preview: false, camera: ArgCamera::Simple, samples: 4 };

    group.bench_function("production_50x50", |b| {
        b.iter(|| {
            let scene =
                CornellBox::new(args_production.width, args_production.height, &args_production);
            let renderer = Renderer::new(scene, &args_production);
            black_box(renderer.render())
        });
    });

    group.finish();
}

criterion_group!(benches, bench_simple, bench_renderer_100x100, bench_renderer_quality_comparison);
criterion_main!(benches);
