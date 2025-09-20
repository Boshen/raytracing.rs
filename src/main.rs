//! Ray tracing executable for rendering 3D scenes.
//!
//! This binary provides a command-line interface for rendering scenes
//! using the raytracing library. It supports various rendering modes
//! including preview mode for fast iterations.

// Custom memory allocators for better performance
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

/// Main entry point for the ray tracer.
///
/// Parses command-line arguments, sets up the scene,
/// performs the render, and saves the output image.
fn main() -> Result<()> {
    // Parse command-line arguments
    let args = args().run();

    // Validate arguments before proceeding
    args.validate().map_err(RayTracingError::ConfigError)?;

    // Initialize the scene and renderer
    println!("Initializing scene...");
    let scene = CornellBox::new(args.height, args.height, &args);
    let renderer = Renderer::new(scene, &args);

    // Display rendering configuration
    println!("Config: {:?}", &args);
    println!(
        "Starting render of {}x{} image with {} samples per pixel...",
        args.width,
        args.height,
        if args.preview { PREVIEW_SAMPLES } else { args.samples }
    );

    // Perform the actual rendering
    let now = Instant::now();
    let pixels = renderer.render();
    let duration = now.elapsed();

    println!("Render completed in {}.{:03}s", duration.as_secs(), duration.subsec_millis());

    // Save the rendered image
    println!("Saving image...");
    flip_horizontal(
        &RgbImage::from_vec(args.width, args.height, pixels.iter().flat_map(to_rgb).collect())
            .unwrap(),
    )
    .save("output.png")?;

    println!("Image saved as output.png");

    Ok(())
}
