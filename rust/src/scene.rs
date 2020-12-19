use image::Rgb;
use nalgebra::Dot;
use rayon::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

use crate::light::Light;
use crate::model::{Color, Hittable, Model, Vec3};
use crate::ray::Ray;
use crate::sampler::get_unit_square_sampler;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub focal_length: u32,
    pub camera: Vec3,
    pub lights: Vec<Light>,
    pub models: Vec<Model>,
    pub sample_points_sqrt: u32,
}

impl Scene {
    pub fn algorithm(&self) -> Vec<(u32, u32, Rgb<u8>)> {
        let mut arr = (0..self.width)
            .into_iter()
            .flat_map(move |i| {
                return (0..self.height)
                    .into_iter()
                    .map(move |j| (i, j, Rgb([0, 0, 0])));
            })
            .collect::<Vec<(u32, u32, Rgb<u8>)>>();

        arr.par_iter_mut().for_each(|t| {
            let (i, j, _c) = t;
            let x = (*i as f64) - (self.width as f64) / 2.0;
            let y = (*j as f64) - (self.height as f64) / 2.0;
            let color = self.antialias(x, y);
            let rgb = Rgb([
                self.to_rgb(color.x),
                self.to_rgb(color.y),
                self.to_rgb(color.z),
            ]);
            t.2 = rgb;
        });
        return arr;
    }

    fn to_rgb(&self, x: f64) -> u8 {
        (x * 255.0).min(255.0).round().max(0.0) as u8
    }

    fn antialias(&self, x: f64, y: f64) -> Color {
        return get_unit_square_sampler(self.sample_points_sqrt)
            .fold(Color::new(0.0, 0.0, 0.0), |color, (dx, dy)| {
                color.add(self.get_color(x + dx, y + dy))
            })
            .div(self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64);
    }

    fn get_color(&self, x: f64, y: f64) -> Color {
        let d = Vec3::new(x, y, self.focal_length as f64);
        let ray = Ray {
            origin: self.camera,
            dir: d,
        };
        return self.trace(&ray, &Color::new(0.0, 0.0, 0.0), 0);
    }

    fn trace(&self, ray: &Ray, color: &Color, depth: u64) -> Color {
        let mut min_distance = f64::INFINITY;
        let mut intersection: Option<(f64, &Model, &Box<dyn Hittable>)> = None;
        for m in self.models.iter() {
            if m.aabb.intersects(&ray) {
                for t in m.hittables.iter() {
                    if let Some(distance) = t.intersects(ray) {
                        if distance < min_distance {
                            min_distance = distance;
                            intersection = Some((distance, m, &t))
                        }
                    }
                }
            }
        }

        match intersection {
            None => Color::new(0.0, 0.0, 0.0),
            Some((distance, model, hittable)) => {
                let point = ray.get_point(distance);
                let shade_color = self
                    .lights
                    .iter()
                    .map(|l| l.shade(&ray, &point, &model, &hittable, &self.models))
                    .fold(Color::new(0.0, 0.0, 0.0), |a, b| a.add(b));
                let reflection_color =
                    self.calc_reflection_color(&ray, &point, &model, &hittable, color, depth);
                return shade_color.add(reflection_color);
            }
        }
    }

    fn calc_reflection_color(
        &self,
        ray: &Ray,
        point: &Vec3,
        model: &Model,
        hittable: &Box<dyn Hittable>,
        color: &Color,
        depth: u64,
    ) -> Color {
        if depth > 3 {
            return *color;
        }
        let reflection = model.material.reflection;
        if reflection == 0.0 {
            return *color;
        }
        let normal = hittable.normal(&point);
        let reflect_dir = 2.0 * ray.dir.dot(&normal);
        let reflect_ray = Ray {
            origin: *point,
            dir: ray.dir.sub(normal.mul(reflect_dir)),
        };
        let reflection_color = self.trace(&reflect_ray, &color, depth + 1);
        return reflection_color.mul(reflection).add(color);
    }
}
