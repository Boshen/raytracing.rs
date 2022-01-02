#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::similar_names)]

use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

use clap::Parser;
use image::RgbImage;
use nalgebra::Point3;

use raytracing::args::{ArgCamera, Args};
use raytracing::asset::Asset;
use raytracing::camera::{Camera, Setting, Simple, ThinLens};
use raytracing::color::to_rgb;
use raytracing::geometric_object::BvhNode;
use raytracing::light::{Ambient, AmbientOcculuder, Light};
use raytracing::model::Vec3;
use raytracing::view_plane::ViewPlane;
use raytracing::world::World;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let asset = Asset::new("./assets/cornell_box.obj");

    let ambient_light = Ambient {
        ls: 0.1,
        cl: Vec3::new(1.0, 1.0, 1.0),
    };

    let ambient_occuluder: Arc<dyn Light> =
        Arc::new(AmbientOcculuder::new(1.0, Vec3::new(1.0, 1.0, 1.0)));
    let mut lights = asset.lights;
    lights.push(ambient_occuluder);
    for light in &mut lights {
        if let Some(l) = Arc::get_mut(light) {
            l.set_sample_points_sqrt(if args.preview { 1 } else { 8 });
        }
    }

    let hres = 500;
    let vres = 500;
    let vp = ViewPlane {
        hres,
        vres,
        pixel_size: 1.0,
    };

    let world = World {
        vp,
        bvh: BvhNode::construct(asset.geometries),
        lights,
        ambient_light,
        max_depth: 15,
    };

    let camera_setting = Setting::new(
        Point3::new(0.0, 0.0, -3.0),
        Point3::new(0.0, 0.0, 0.0),
        500.0,
    );

    let camera: Box<dyn Camera> = match args.camera {
        ArgCamera::Simple => Box::new(Simple::new(camera_setting)),
        ArgCamera::ThinLens => Box::new(ThinLens::new(camera_setting, 0.001, 500.0)),
    };

    let now = Instant::now();

    let pixels = camera
        .render_scene(&world)
        .iter()
        .flat_map(to_rgb)
        .collect();
    let duration = now.elapsed();

    RgbImage::from_vec(hres, vres, pixels)
        .unwrap()
        .save("output.png")?;

    println!(
        "Time Elapased: {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    Ok(())
}
