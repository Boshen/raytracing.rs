#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::error::Error;
use std::time::Instant;

use clap::Parser;
use image::imageops::flip_horizontal;
use image::RgbImage;

use raytracing::args::Args;
use raytracing::color::to_rgb;
use raytracing::renderer::Renderer;
use raytracing::scene::CornellBox;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let view_width = 500;
    let view_height = 500;
    let scene = CornellBox::new(view_width, view_height, &args);
    let renderer = Renderer::new(scene, &args);

    let now = Instant::now();
    let pixels = renderer.render();
    let duration = now.elapsed();

    println!(
        "Render Time Elapased: {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    let now = Instant::now();
    flip_horizontal(
        &RgbImage::from_vec(
            view_width,
            view_height,
            pixels.iter().flat_map(to_rgb).collect(),
        )
        .unwrap(),
    )
    .save("output.png")?;
    let duration = now.elapsed();

    println!(
        "Save Time Elapased: {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    Ok(())
}
