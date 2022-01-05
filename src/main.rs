#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::error::Error;
use std::time::Instant;

use clap::Parser;
use image::imageops::flip_horizontal;
use image::RgbImage;

use raytracing::args::Args;
use raytracing::color::to_rgb;
use raytracing::counter;
use raytracing::renderer::Renderer;
use raytracing::scene::CornellBox;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let scene = CornellBox::new(args.height, args.height, &args);
    let renderer = Renderer::new(scene, &args);

    println!("Config: {:?}", &args);

    let now = Instant::now();
    let pixels = renderer.render();
    let duration = now.elapsed();

    println!(
        "Render Time Elapased: {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    flip_horizontal(
        &RgbImage::from_vec(
            args.width,
            args.height,
            pixels.iter().flat_map(to_rgb).collect(),
        )
        .unwrap(),
    )
    .save("output.png")?;

    counter::print_count();

    Ok(())
}
