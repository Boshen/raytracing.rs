use image::{RgbImage};

mod model;
mod ray;
mod models;
mod scene;
mod light;
mod aabb;

use crate::model::{Vec3};
use crate::models::{get_models};
use crate::scene::{Scene};
use crate::light::{Light, LightData};

fn main() {
    let width = 500;
    let height = 500;

    let lights = vec![
        Light::Ambient(LightData {
            radiance: 1.0,
            color: Vec3::new(0.2, 0.2, 0.2),
            location: Vec3::new(0.0, 0.0, 0.0)
        }),
        Light::Directional(LightData{
            radiance: 1.0,
            color: Vec3::new(1.0, 1.0, 1.0),
            location: Vec3::new(0.0, 0.0, -1.0)
        }),
        Light::Point(LightData{
            radiance: 3.0,
            color: Vec3::new(1.0, 1.0, 1.0),
            location: Vec3::new(0.0, -1.0, 0.0)
        }),
    ];

    let scene = Scene {
        width: width,
        height: height,
        focal_length: width,
        camera: Vec3::new(0.0, 0.0, -3.0),
        models: get_models(),
        lights: lights,
    };

    let mut image = RgbImage::new(width, height);
    scene.algorithm(&mut image);
    image.save("output.png").unwrap();
}
