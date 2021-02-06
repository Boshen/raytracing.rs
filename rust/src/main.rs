#![allow(dead_code)]
#![allow(clippy::many_single_char_names)]

use image::RgbImage;
use std::error::Error;

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
use crate::camera::{Camera, ThinLensCamera};
use crate::color::to_rgb;
use crate::light::{AmbientLight, AmbientOcculuder, LightEnum};
use crate::model::Vec3;
use crate::view_plane::ViewPlane;
use crate::world::World;

fn main() -> Result<(), Box<dyn Error>> {
    let asset = Asset::new("../assets/cornell_box.obj");

    let ambient_light = AmbientLight {
        ls: 0.1,
        cl: Vec3::new(1.0, 1.0, 1.0),
    };

    let lights: Vec<LightEnum> = vec![LightEnum::from(AmbientOcculuder {
        ls: 1.0,
        cl: Vec3::new(1.0, 1.0, 1.0),
        sample_points_sqrt: 4,
    })];

    let vp = ViewPlane {
        hres: 500,
        vres: 500,
        pixel_size: 1.0,
    };

    let world = World {
        vp,
        models: asset.models,
        lights: lights
            .into_iter()
            .chain(asset.lights.into_iter())
            .collect::<Vec<LightEnum>>(),
        ambient_light,
    };

    let camera = ThinLensCamera::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(0.0, 0.0, 0.0), 500.0);

    let pixels = camera
        .render_scene(&world)
        .iter()
        .flat_map(to_rgb)
        .collect();

    RgbImage::from_vec(vp.hres, vp.vres, pixels)
        .unwrap()
        .save("output.png")?;

    Ok(())
}
