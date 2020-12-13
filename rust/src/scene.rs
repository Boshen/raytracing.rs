use image::{RgbImage, Rgb};
use nalgebra::{Vector3};
use std::ops::Add;
use std::ops::Mul;

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
                let color = self.antialias(x, y);
                let rgb = Rgb([
                    self.to_rgb(color.x),
                    self.to_rgb(color.y),
                    self.to_rgb(color.z),
                ]);
                image.put_pixel(i, j, rgb);
            }
        }
    }

    fn to_rgb(&self, x: f64) -> u8 {
        (x * 255.0).min(255.0).round().max(0.0) as u8
    }

    fn antialias(&self, x: f64, y: f64) -> Vector3<f64> {
        let n = 5; // sample points
        let mut color = Vector3::new(0.0, 0.0, 0.0);
        for i in 0..n {
            for j in 0..n {
                let dx = (i as f64 + 0.5) / n as f64;
                let dy = (j as f64 + 0.5) / n as f64;
                color = color.add(self.get_color(x + dx, y + dy))
            }
        }
        return color.mul(1.0 / (n as f64 * n as f64))
    }

    fn get_color(&self, x: f64, y: f64) -> Vector3<f64> {
        let d = Vector3::new(x, y, self.focal_length as f64);
        let ray = Ray {start: self.camera, direction: d};
        return self.trace(&ray)
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
                    .map(|l| l.shade(&ray, ray.get_point(distance), &model, &hittable, &self.models))
                    .fold(Vector3::new(0.0, 0.0, 0.0), |a, b| a.add(b))
            }
        }
    }
}
