use nalgebra::{Cross, Norm};
use num_traits::identities::Zero;
use rayon::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

use crate::color::{to_rgb, Color};
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
        let empty = Vec3::zero();
        Camera {
            sample_points_sqrt: 1,
            up: Vec3::new(0.0, 1.0, 0.0),
            eye: empty,
            u: empty,
            v: empty,
            w: empty,
            view_distance: 500.0,
        }
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
        Camera {
            eye,
            w,
            u,
            v,
            view_distance,
            ..camera
        }
    }

    pub fn render_scence(&self, world: &World) -> Vec<u8> {
        let hres = world.vp.hres;
        let vres = world.vp.vres;
        let pixel_size = world.vp.pixel_size;
        (0..(world.vp.hres * world.vp.vres))
            .into_par_iter()
            .flat_map(|n| {
                let (i, j) = (n % hres, n / hres);
                let x = pixel_size * (i as f64 - 0.5 * (hres as f64 - 1.0));
                let y = pixel_size * (j as f64 - 0.5 * (vres as f64 - 1.0));
                to_rgb(&self.antialias(world, x, y))
            })
            .collect()
    }

    fn antialias(&self, world: &World, x: f64, y: f64) -> Color {
        get_unit_square_sampler(self.sample_points_sqrt)
            .map(|(dx, dy)| world.trace(&self.get_direction(x + dx, y + dy), 0))
            .fold(Vec3::zero(), |v1, v2| v1.add(v2))
            .div(self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64)
    }

    fn get_direction(&self, x: f64, y: f64) -> Ray {
        let dir = self
            .u
            .mul(x)
            .add(self.v.mul(y))
            .sub(self.w.mul(self.view_distance))
            .normalize();
        Ray::new(self.eye, dir)
    }
}
