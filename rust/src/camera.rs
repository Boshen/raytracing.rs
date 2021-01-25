use nalgebra::{Cross, Norm};
use rayon::prelude::*;
use std::ops::{Add, Div, Sub};

use crate::model::{Color, Vec3};
use crate::ray::Ray;
use crate::sampler::get_unit_square_sampler;
use crate::world::World;

pub struct Camera<'a> {
    pub eye: Vec3,
    pub lookat: Vec3,
    pub up: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub sample_points_sqrt: u32,
    pub world: &'a World,
}

impl Camera<'_> {
    pub fn new(
        world: &'_ World,
        eye: Vec3,
        lookat: Vec3,
        up: Vec3,
        sample_points_sqrt: u32,
    ) -> Camera<'_> {
        let w = eye.sub(lookat).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u).normalize();
        return Camera {
            eye,
            lookat,
            up,
            u,
            v,
            w,
            sample_points_sqrt,
            world,
        };
    }

    pub fn render_scence(&self) -> Vec<(u32, u32, (u8, u8, u8))> {
        let width = self.world.width;
        let height = self.world.height;
        return (0..(width * height))
            .into_par_iter()
            .map(|n| {
                let (i, j) = (n % width, n / width);
                let x = (i as f64) - (width as f64) / 2.0;
                let y = (j as f64) - (height as f64) / 2.0;
                let color = self.tone_mapping(self.antialias(x, y));
                let rgb = (
                    self.to_rgb(color.x),
                    self.to_rgb(color.y),
                    self.to_rgb(color.z),
                );
                return (i, j, rgb);
            })
            .collect();
    }

    fn antialias(&self, x: f64, y: f64) -> Color {
        return get_unit_square_sampler(self.sample_points_sqrt)
            .fold(Color::new(0.0, 0.0, 0.0), |color, (dx, dy)| {
                color.add(self.get_color(x + dx, y + dy))
            })
            .div(self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64);
    }

    fn tone_mapping(&self, color: Color) -> Color {
        let max = color.x.max(color.y).max(color.z).max(1.0);
        return color.div(max);
    }

    fn to_rgb(&self, x: f64) -> u8 {
        (x * 255.0).min(255.0).round().max(0.0) as u8
    }

    fn get_color(&self, x: f64, y: f64) -> Color {
        let focal_length = 500; //self.wo
        let d = Vec3::new(x, y, focal_length as f64);
        let ray = Ray::new(self.eye, d);
        return self.world.trace(&ray, 0);
    }
}
