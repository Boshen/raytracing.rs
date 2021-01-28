use image::RgbImage;
use std::error::Error;

mod aabb;
mod asset;
mod brdf;
mod camera;
mod hittable;
mod light;
mod material;
mod model;
mod ray;
mod sampler;
mod world;

use crate::asset::Asset;
use crate::camera::Camera;
use crate::light::{
    AmbientLight, AmbientOcculuder, AreaLight, DirectionalLight, Light, PointLight,
};
use crate::model::Vec3;
use crate::world::World;

fn main() -> Result<(), Box<dyn Error>> {
    let asset = Asset::new("../assets/cornell_box.obj");

    let ambient_light = Box::new(AmbientLight {
        ls: 0.1,
        cl: Vec3::new(1.0, 1.0, 1.0),
    });

    let lights: Vec<Box<dyn Light>> = vec![
        Box::new(AmbientOcculuder {
            ls: 1.0,
            cl: Vec3::new(1.0, 1.0, 1.0),
            sample_points_sqrt: 16,
        }),
        Box::new(DirectionalLight {
            ls: 1.0,
            cl: Vec3::new(1.0, 1.0, 1.0),
            direction: Vec3::new(0.0, 1.0, 0.0),
        }),
        Box::new(PointLight {
            ls: 1.0,
            cl: Vec3::new(1.0, 1.0, 1.0),
            location: Vec3::new(0.0, -1.0, 0.0),
        }),
        Box::new(AreaLight {
            ls: 2.0,
            cl: Vec3::new(1.0, 1.0, 1.0),
            location: Vec3::new(0.0, -1.0, 0.0),
            width: 75.0 / 255.0,
            height: 75.0 / 255.0,
            sample_points_sqrt: 5,
        }),
    ];

    let world = World {
        width: 500,
        height: 500,
        models: asset.models,
        lights,
        ambient_light,
    };

    let camera = Camera::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(0.0, 0.0, 0.0), 500.0);

    let pixels = camera
        .render_scence(&world)
        .into_iter()
        .flat_map(|(r, g, b)| vec![r, g, b])
        .collect();

    RgbImage::from_vec(world.width, world.height, pixels)
        .unwrap()
        .save("output.png")?;

    Ok(())
}
