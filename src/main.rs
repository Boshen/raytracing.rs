#[cfg(all(not(target_env = "msvc"), not(target_os = "windows")))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::time::Instant;

use image::{RgbImage, imageops::flip_horizontal};
use raytracing::{
    args::args,
    color::to_rgb,
    error::{RayTracingError, Result},
    renderer::{PREVIEW_SAMPLES, Renderer},
    scene::CornellBox,
};

fn main() -> Result<()> {
    let args = args().run();

    // Validate arguments
    args.validate().map_err(RayTracingError::ConfigError)?;

    println!("Initializing scene...");
    let scene = CornellBox::new(args.height, args.height, &args);
    let renderer = Renderer::new(scene, &args);

    println!("Config: {:?}", &args);
    println!(
        "Starting render of {}x{} image with {} samples per pixel...",
        args.width,
        args.height,
        if args.preview { PREVIEW_SAMPLES } else { args.samples }
    );

    let now = Instant::now();
    let pixels = renderer.render();
    let duration = now.elapsed();

    println!("Render completed in {}.{:03}s", duration.as_secs(), duration.subsec_millis());

    println!("Saving image...");
    flip_horizontal(
        &RgbImage::from_vec(args.width, args.height, pixels.iter().flat_map(to_rgb).collect())
            .unwrap(),
    )
    .save("output.png")?;

    println!("Image saved as output.png");

    Ok(())
}
