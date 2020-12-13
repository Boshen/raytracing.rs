use image::{RgbImage, Rgb};
use nalgebra::{Vector3};
use std::ops::Add;

use crate::model::{Model, Hittable};
use crate::light::{Light};
use crate::ray::{Ray};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub focal_length: u32,
    pub camera: Vector3<f64>,
    pub lights: Vec<Box<dyn Light>>,
    pub models: Vec<Model>
}

impl Scene {
    pub fn algorithm(&self, image: &mut RgbImage) {
        for i in 0..self.width {
            for j in 0..self.height {
                let x = (i as f64)  - (self.width as f64) / 2.0;
                let y = (j as f64) - (self.height as f64) / 2.0;
                let d = Vector3::new(x, y, self.focal_length as f64);
                let ray = Ray {start: self.camera, direction: d};
                let color = self.trace(&ray);
                let rgb = Rgb([
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8
                ]);
                image.put_pixel(i, j, rgb);
            }
        }
    }

    fn trace(&self, ray: &Ray) -> Vector3<f64> {
        let mut min_distance = f64::INFINITY;
        let mut intersection: Option<(f64, &Model, &Box<dyn Hittable>)> = None;
        for m in self.models.iter() {
            for t in m.hittables.iter() {
                if let Some(distance) = t.intersects(ray) {
                    if distance < min_distance {
                        min_distance = distance;
                        intersection = Some((distance, m, &t))
                    }
                }
            }
        }

        match intersection {
            None => Vector3::new(0.0, 0.0, 0.0),
            Some((distance, model, hittable)) => {
                self.lights
                    .iter()
                    .map(|l| l.shade(&ray, ray.get_point(distance), &model, &hittable))
                    .fold(Vector3::new(0.0, 0.0, 0.0), |a, b| a.add(b))
            }
        }
    }
}
