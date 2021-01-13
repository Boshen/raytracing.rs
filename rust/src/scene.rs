use image::Rgb;
use nalgebra::Dot;
use rayon::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

use crate::hittable::Hittable;
use crate::light::Light;
use crate::model::{Color, Model, Vec3};
use crate::ray::{Ray, RayHit};
use crate::sampler::get_unit_square_sampler;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub focal_length: u32,
    pub camera: Vec3,
    pub lights: Vec<Box<dyn Light>>,
    pub models: Vec<Model>,
    pub sample_points_sqrt: u32,
}

impl Scene {
    pub fn algorithm(&self) -> Vec<(u32, u32, Rgb<u8>)> {
        return (0..(self.width * self.height))
            .into_par_iter()
            .map(|n| {
                let (i, j) = (n % self.width, n / self.width);
                let x = (i as f64) - (self.width as f64) / 2.0;
                let y = (j as f64) - (self.height as f64) / 2.0;
                let color = self.tone_mapping(self.antialias(x, y));
                let rgb = Rgb([
                    self.to_rgb(color.x),
                    self.to_rgb(color.y),
                    self.to_rgb(color.z),
                ]);
                return (i, j, rgb);
            })
            .collect();
    }

    fn tone_mapping(&self, color: Color) -> Color {
        let max = color.x.max(color.y).max(color.z);
        if max > 1.0 {
            return color.div(max);
        } else {
            return color;
        }
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
        let intersection = self
            .models
            .iter()
            .filter(|model| model.aabb.intersects(&ray))
            .flat_map(|model| {
                model
                    .hittables
                    .iter()
                    .map(move |hittable| (model, hittable))
            })
            .filter_map(|(model, hittable)| {
                hittable.intersects(ray).map(|dist| (dist, model, hittable))
            })
            .min_by(|t1, t2| (t1.0).partial_cmp(&t2.0).expect("Tried to compare a NaN"));

        match intersection {
            None => Color::new(0.0, 0.0, 0.0),
            Some((distance, model, hittable)) => {
                let point = ray.get_point(distance);
                let shade_color = self
                    .lights
                    .iter()
                    .map(|l| {
                        l.shade(&RayHit {
                            ray: Box::new(ray),
                            hit_point: point,
                            material: Box::new(model.material),
                            hittable: &hittable,
                            models: Box::new(&self.models),
                        })
                    })
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
