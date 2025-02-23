#[cfg(all(not(target_env = "msvc"), not(target_os = "windows")))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::{error::Error, time::Instant};

use image::{RgbImage, imageops::flip_horizontal};
use raytracing::{args::args, color::to_rgb, renderer::Renderer, scene::CornellBox};

fn main() -> Result<(), Box<dyn Error>> {
    let args = args().run();

    let scene = CornellBox::new(args.height, args.height, &args);
    let renderer = Renderer::new(scene, &args);

    println!("Config: {:?}", &args);

    let now = Instant::now();
    let pixels = renderer.render();
    let duration = now.elapsed();

    println!("Render Time Elapsed: {}.{}s", duration.as_secs(), duration.subsec_millis());

    flip_horizontal(
        &RgbImage::from_vec(args.width, args.height, pixels.iter().flat_map(to_rgb).collect())
            .unwrap(),
    )
    .save("output.png")?;

    Ok(())
}
