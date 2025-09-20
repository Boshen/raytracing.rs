//! Ray tracing executable for rendering 3D scenes.
//!
//! This binary provides a command-line interface for rendering scenes
//! using the raytracing library. It supports various rendering modes
//! including preview mode for fast iterations.
//!
//! ## Usage Examples
//!
//! ```bash
//! # Basic render with default settings
//! cargo run --release
//!
//! # Quick preview mode (1 sample, depth 1)
//! cargo run --release -- --preview
//!
//! # High quality render (16 samples per pixel)
//! cargo run --release -- --samples 16
//!
//! # Render with thin lens camera (depth of field)
//! cargo run --release -- --camera thin-lens
//!
//! # Custom resolution
//! cargo run --release -- --width 1920 --height 1080
//! ```
//!
//! ## Performance Tips
//!
//! - Always use `--release` for rendering (10-100x faster)
//! - Use `--preview` for quick iterations while developing
//! - Higher sample counts reduce noise but increase render time linearly
//! - The BVH acceleration structure handles complex scenes efficiently

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
    color::{Color, to_rgb},
    error::{RayTracingError, Result},
    renderer::{PREVIEW_SAMPLES, Renderer},
    scene::CornellBox,
};

/// Main entry point for the ray tracer.
///
/// Workflow:
/// 1. Parse and validate command-line arguments
/// 2. Set up the scene with geometry, lights, and camera
/// 3. Create renderer with appropriate sampling strategy
/// 4. Perform parallel ray tracing
/// 5. Save the result as PNG
///
/// The renderer uses Rayon for parallel pixel processing,
/// making it scale well with available CPU cores.
fn main() -> Result<()> {
    // Parse command-line arguments
    let args = args().run();

    // Validate arguments before proceeding
    args.validate().map_err(RayTracingError::ConfigError)?;

    // Initialize the scene and renderer
    println!("🎬 Initializing scene...");
    let scene = create_scene(&args);
    let renderer = Renderer::new(scene, &args);

    // Display rendering configuration
    print_config(&args);

    // Perform the actual rendering with timing
    let now = Instant::now();
    let pixels = renderer.render();
    let duration = now.elapsed();

    print_stats(duration, &args);

    // Save the rendered image
    save_image(&pixels, &args)?;

    Ok(())
}

/// Creates the appropriate scene based on configuration.
///
/// Currently only supports Cornell Box, but this is where
/// you'd add scene selection logic for multiple scenes.
fn create_scene(args: &raytracing::args::Args) -> CornellBox {
    // Note: Using height for both dimensions creates a square image
    // This is intentional for the Cornell Box scene
    CornellBox::new(args.height, args.height, args)
}

/// Prints the rendering configuration in a user-friendly format.
fn print_config(args: &raytracing::args::Args) {
    println!("📋 Configuration:");
    println!("  Resolution: {}x{}", args.width, args.height);
    println!("  Camera: {:?}", args.camera);
    println!("  Samples per pixel: {}", if args.preview { PREVIEW_SAMPLES } else { args.samples });
    println!("  Max ray depth: {}", if args.preview { 1 } else { 5 });
    println!("  Mode: {}", if args.preview { "Preview" } else { "Production" });
}

/// Prints rendering statistics after completion.
fn print_stats(duration: std::time::Duration, args: &raytracing::args::Args) {
    let total_rays = args.width
        * args.height
        * u32::from(if args.preview { PREVIEW_SAMPLES } else { args.samples });
    let rays_per_sec = total_rays as f64 / duration.as_secs_f64();

    println!("✅ Render completed in {}.{:03}s", duration.as_secs(), duration.subsec_millis());
    println!("   Total rays: {}", total_rays);
    println!("   Performance: {:.0} rays/second", rays_per_sec);
}

/// Saves the rendered pixels as a PNG image.
///
/// The image is flipped horizontally to match the expected orientation
/// (ray tracer uses a different coordinate system than image formats).
fn save_image(pixels: &[Color], args: &raytracing::args::Args) -> Result<()> {
    println!("💾 Saving image...");

    flip_horizontal(
        &RgbImage::from_vec(args.width, args.height, pixels.iter().flat_map(to_rgb).collect())
            .unwrap(),
    )
    .save("output.png")?;

    println!("📸 Image saved as output.png");
    Ok(())
}
