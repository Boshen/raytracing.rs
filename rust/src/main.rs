use image::RgbImage;

mod aabb;
mod light;
mod model;
mod models;
mod ray;
mod sampler;
mod scene;

use crate::light::{AmbientLight, AmbientOcculuder, AreaLight, DirectionalLight, Light};
use crate::model::Vec3;
use crate::models::get_models;
use crate::scene::Scene;

fn main() {
    let width = 500;
    let height = 500;

    let lights = vec![
        Light::Ambient(AmbientLight {
            radiance: 0.1,
            color: Vec3::new(1.0, 1.0, 1.0),
        }),
        Light::AmbientOcculuder(AmbientOcculuder {
            radiance: 0.3,
            color: Vec3::new(1.0, 1.0, 1.0),
            sample_points_sqrt: 16,
        }),
        Light::Directional(DirectionalLight {
            radiance: 1.0,
            color: Vec3::new(1.0, 1.0, 1.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
        }),
        Light::Area(AreaLight {
            radiance: 1.5,
            color: Vec3::new(1.0, 1.0, 1.0),
            location: Vec3::new(0.0, -1.0, 0.0),
            width: 75.0 / 255.0,
            height: 75.0 / 255.0,
            sample_points_sqrt: 5,
        }),
    ];

    let scene = Scene {
        width: width,
        height: height,
        focal_length: width,
        camera: Vec3::new(0.0, 0.0, -3.0),
        models: get_models(),
        lights: lights,
        sample_points_sqrt: 1,
    };

    let mut image = RgbImage::new(width, height);
    scene
        .algorithm()
        .into_iter()
        .for_each(|(i, j, rgb)| image.put_pixel(i, j, rgb));
    image.save("output.png").unwrap();
}
