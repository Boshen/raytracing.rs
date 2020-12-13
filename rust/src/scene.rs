use image::{RgbImage, Rgb};
use nalgebra::{Vector3};

use crate::ray::{Ray};
use crate::model::{Triangle};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub focal_length: u32,
    pub camera: Vector3<f64>,
    pub models: Vec<Triangle>
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
        let mut hit_model: Option<&Triangle> = None;
        for m in &self.models {
            match m.intersects(ray) {
                None => (),
                Some(t) => {
                    if t < min_distance {
                        min_distance = t;
                        hit_model = Some(&m)
                    }
                }
            }
        }

        match hit_model {
            None => Vector3::new(0.0, 0.0, 0.0),
            Some(_m) => Vector3::new(1.0, 1.0, 1.0),
        }
    }
}
