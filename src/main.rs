#![allow(dead_code)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::many_single_char_names,
    clippy::module_name_repetitions,
    clippy::similar_names
)]

use image::RgbImage;
use nalgebra::Point3;
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

mod aabb;
mod asset;
mod brdf;
mod camera;
mod color;
mod geometric_object;
mod light;
mod material;
mod model;
mod ray;
mod sampler;
mod view_plane;
mod world;

use crate::asset::Asset;
use crate::camera::{Camera, CameraSetting, ThinLensCamera};
use crate::color::to_rgb;
use crate::geometric_object::BvhNode;
use crate::light::{AmbientLight, AmbientOcculuder, Light};
use crate::model::Vec3;
use crate::view_plane::ViewPlane;
use crate::world::World;

fn main() -> Result<(), Box<dyn Error>> {
    let preview = env::args().any(|x| x == "--preview");
    println!("Preview Mode: {:?}", preview);

    let asset = Asset::new("./assets/cornell_box.obj");

    let ambient_light = AmbientLight {
        ls: 0.1,
        cl: Vec3::new(1.0, 1.0, 1.0),
    };

    let ambient_occuluder: Arc<dyn Light + Send + Sync> =
        Arc::new(AmbientOcculuder::new(1.0, Vec3::new(1.0, 1.0, 1.0)));
    let mut lights = asset.lights;
    lights.push(ambient_occuluder);
    for light in &mut lights {
        if let Some(l) = Arc::get_mut(light) {
            l.set_sample_points_sqrt(if preview { 1 } else { 4 });
        }
    }

    let hres = 500;
    let vres = 500;
    let vp = ViewPlane {
        hres,
        vres,
        pixel_size: 1.0,
    };

    let len = asset.geometries.len();
    let world = World {
        vp,
        bvh: Arc::new(BvhNode::new(asset.geometries, 0, len)),
        lights,
        ambient_light,
        materials: asset.materials,
        max_depth: 15,
    };

    let camera = ThinLensCamera {
        setting: CameraSetting::new(
            Point3::new(0.0, 0.0, -3.0),
            Point3::new(0.0, 0.0, 0.0),
            500.0,
        ),
        lens_radius: 0.001, // 0 = simple camera with no blur
        focal_plane_distance: 500.0,
    };

    let now = Instant::now();
    let pixels = camera
        .render_scene(&world)
        .iter()
        .flat_map(to_rgb)
        .collect();
    let duration = now.elapsed();
    println!(
        "Time Elapased: {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    RgbImage::from_vec(hres, vres, pixels)
        .unwrap()
        .save("output.png")?;

    Ok(())
}
