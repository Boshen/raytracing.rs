use image::{RgbImage, Rgb};
use nalgebra::{Vector3, Dot};
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use rayon::prelude::*;

use crate::model::{Model, Hittable};
use crate::light::{Light};
use crate::ray::{Ray};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub focal_length: u32,
    pub camera: Vector3<f64>,
    pub lights: Vec<Light>,
    pub models: Vec<Model>
}

impl Scene {
    pub fn algorithm(&self, image: &mut RgbImage) {
        let mut xys: Vec<(u32, u32, Vector3<f64>)> = vec![];
        for i in 0..self.width {
            for j in 0..self.width {
                xys.push((i, j, Vector3::new(0.0, 0.0, 0.0)))
            }
        }

        xys.par_iter_mut().for_each(|t| {
            let (i, j, _c) = t;
            let x = (*i as f64)  - (self.width as f64) / 2.0;
            let y = (*j as f64) - (self.height as f64) / 2.0;
            let color = self.antialias(x, y);
            t.2 = color;
        });

        xys.iter().for_each(|(i, j, color)| {
            let rgb = Rgb([self.to_rgb(color.x), self.to_rgb(color.y), self.to_rgb(color.z)]);
            image.put_pixel(*i, *j, rgb);
        });
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
        return self.trace(&ray, Vector3::new(0.0, 0.0, 0.0), 0)
    }

    fn trace(&self, ray: &Ray, color: Vector3<f64>, depth: u64) -> Vector3<f64> {
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
                let point = ray.get_point(distance);
                let shade_color = self.lights
                    .iter()
                    .map(|l| l.shade(&ray, &point, &model, &hittable, &self.models))
                    .fold(Vector3::new(0.0, 0.0, 0.0), |a, b| a.add(b));
                let reflection_color = self.calc_reflection_color(&ray, &point, &model, &hittable, color, depth);
                return shade_color.add(reflection_color)
            }
        }
    }

  fn calc_reflection_color(&self, ray: &Ray, point: &Vector3<f64>, model: &Model, hittable: &Box<dyn Hittable>, color: Vector3<f64>, depth: u64) -> Vector3<f64> {
    if depth > 3 {
      return color
    }
    let reflection = model.material.reflection;
    if reflection == 0.0 {
      return color
    }
    let normal = hittable.normal(&point);
    let reflect_dir = 2.0 * ray.direction.dot(&normal);
    let reflect_ray = Ray{start: *point, direction: ray.direction.sub(normal.mul(reflect_dir))};
    let reflection_color = self.trace(&reflect_ray, color, depth + 1);
    return reflection_color.mul(reflection).add(color);
  }
}
