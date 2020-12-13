use image::{RgbImage};
use nalgebra::{Vector3};

mod model;
mod ray;
mod models;
mod scene;

use crate::models::{get_models};
use crate::scene::{Scene};

fn main() {
    let width = 500;
    let height = 500;
    let scene = Scene {
        width: width,
        height: height,
        focal_length: width,
        camera: Vector3::new(0.0, 0.0, -3.0),
        models: get_models(),
    };

    let mut image = RgbImage::new(width, height);
    scene.algorithm(&mut image);
    image.save("output.png").unwrap();
}
