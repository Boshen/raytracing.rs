use nalgebra::{Norm, Point2};
use num_traits::identities::Zero;
use rayon::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

use crate::camera::{Camera, CameraSetting};
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Ray;
use crate::sampler::get_square_sampler;
use crate::world::World;

pub struct SimpleCamera {
    pub setting: CameraSetting,
}

impl Camera for SimpleCamera {
    fn render_scene(&self, world: &World) -> Vec<Color> {
        let hres = world.vp.hres;
        let vres = world.vp.vres;
        let pixel_size = world.vp.pixel_size;
        (0..(world.vp.hres * world.vp.vres))
            .into_par_iter()
            .map(|n| {
                let (i, j) = (n % hres, n / hres);
                let p = Point2::new(
                    pixel_size * (i as f64 - (hres as f64) / 2.0),
                    pixel_size * (j as f64 - (vres as f64) / 2.0),
                );
                get_square_sampler(self.setting.sample_points_sqrt)
                    .map(|dp| {
                        let ray = self.get_ray(p.add(dp.to_vector()));
                        world.trace(&ray, 0)
                    })
                    .fold(Vec3::zero(), |v1, v2| v1.add(v2))
                    .div((self.setting.sample_points_sqrt * self.setting.sample_points_sqrt) as f64)
            })
            .collect()
    }
}

impl SimpleCamera {
    fn get_ray(&self, p: Point2<f64>) -> Ray {
        let dir = self
            .setting
            .u
            .mul(p.x)
            .add(self.setting.v.mul(p.y))
            .sub(self.setting.w.mul(self.setting.view_plane_distance))
            .normalize();
        Ray::new(self.setting.eye, dir)
    }
}
