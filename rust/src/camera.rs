use nalgebra::{clamp, Cross, Norm};
use rayon::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

use crate::color::{tone_mapping, Color};
use crate::model::Vec3;
use crate::ray::Ray;
use crate::sampler::get_unit_square_sampler;
use crate::world::World;

pub struct Camera {
    sample_points_sqrt: u32,
    up: Vec3,
    eye: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    view_distance: f64,
}

impl Default for Camera {
    fn default() -> Camera {
        let empty = Vec3::new(0.0, 0.0, 0.0);
        return Camera {
            sample_points_sqrt: 1,
            up: Vec3::new(0.0, 1.0, 0.0),
            eye: empty,
            u: empty,
            v: empty,
            w: empty,
            view_distance: 500.0,
        };
    }
}

impl Camera {
    pub fn new(eye: Vec3, lookat: Vec3, view_distance: f64) -> Camera {
        let camera = Camera {
            ..Default::default()
        };
        let w = eye.sub(lookat).normalize();
        let u = camera.up.cross(&w).normalize();
        let v = w.cross(&u).normalize();
        return Camera {
            eye,
            w,
            u,
            v,
            view_distance,
            ..camera
        };
    }

    pub fn render_scence(&self, world: &World) -> Vec<(u8, u8, u8)> {
        let width = world.width;
        let height = world.height;
        return (0..(width * height))
            .into_par_iter()
            .map(|n| {
                let (i, j) = (n % width, n / width);
                let x = (i as f64) - (width as f64) / 2.0;
                let y = (j as f64) - (height as f64) / 2.0;
                let color = tone_mapping(&self.antialias(world, x, y));
                return (
                    self.to_rgb(color.x),
                    self.to_rgb(color.y),
                    self.to_rgb(color.z),
                );
            })
            .collect();
    }

    fn antialias(&self, world: &World, x: f64, y: f64) -> Color {
        return get_unit_square_sampler(self.sample_points_sqrt)
            .map(|(dx, dy)| world.trace(&self.get_direction(x + dx, y + dy), 0))
            .fold(Vec3::new(0.0, 0.0, 0.0), |v1, v2| v1.add(v2))
            .div(self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64);
    }

    fn to_rgb(&self, x: f64) -> u8 {
        clamp((x * 255.0).round() as u8, 0, 255) as u8
    }

    fn get_direction(&self, x: f64, y: f64) -> Ray {
        let dir = self
            .u
            .mul(x)
            .add(self.v.mul(y))
            .sub(self.w.mul(self.view_distance))
            .normalize();
        return Ray::new(self.eye, dir);
    }
}
