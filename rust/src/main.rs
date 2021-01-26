use image::{Rgb, RgbImage};
use std::io;

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

fn main() -> io::Result<()> {
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
            location: Vec3::new(278.0, 548.5, 279.5),
            width: 75.0 / 255.0,
            height: 75.0 / 255.0,
            sample_points_sqrt: 5,
        }),
    ];

    let world = World {
        width: 512,
        height: 512,
        models: asset.models,
        lights,
        ambient_light,
    };

    // let camera = Camera::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(0.0, 0.0, 0.0), 500.0);

    let camera = Camera::new(
        Vec3::new(278.0, 273.0, -500.0),
        Vec3::new(278.0, 273.0, 0.0),
    );

    let mut image = RgbImage::new(world.width, world.height);
    camera
        .render_scence(&world)
        .into_iter()
        .for_each(|(i, j, (r, g, b))| image.put_pixel(i, j, Rgb([r, g, b])));
    image.save("output.png").unwrap();

    Ok(())
}
