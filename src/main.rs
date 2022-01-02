#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::similar_names)]

use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

use clap::Parser;
use image::RgbImage;

use raytracing::args::Args;
use raytracing::asset::Asset;
use raytracing::color::to_rgb;
use raytracing::geometric_object::BvhNode;
use raytracing::scene::CornellBox;
use raytracing::world::World;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let view_width = 500;
    let view_height = 500;

    let asset = Asset::new("./assets/cornell_box.obj");
    let mut scene = CornellBox::default();

    let mut lights = asset.lights;
    lights.push(scene.ambient_occuluder.clone());
    for light in &mut lights {
        if let Some(l) = Arc::get_mut(light) {
            l.set_sample_points_sqrt(if args.preview { 1 } else { 8 });
        }
    }

    scene.camera_setting.set_view((view_width, view_height));
    scene
        .camera_setting
        .set_sample_points_sqrt(if args.preview { 1 } else { 8 });

    let world = World {
        bvh: BvhNode::construct(asset.geometries),
        lights,
        ambient_light: scene.ambient_light.clone(),
        max_depth: if args.preview { 1 } else { 5 },
    };

    let now = Instant::now();
    let pixels = scene
        .camera(&args.camera)
        .render_scene(&world)
        .iter()
        .flat_map(to_rgb)
        .collect();
    let duration = now.elapsed();

    println!(
        "Render Time Elapased: {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    let now = Instant::now();
    RgbImage::from_vec(view_width, view_height, pixels)
        .unwrap()
        .save("output.png")?;
    let duration = now.elapsed();

    println!(
        "Save Time Elapased: {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    Ok(())
}
