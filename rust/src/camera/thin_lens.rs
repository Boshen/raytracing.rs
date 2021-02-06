use crate::model::Vec3;
use nalgebra::{Norm, Point2, Vector2};
use num_traits::identities::Zero;
use rayon::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

use crate::camera::{Camera, CameraSetting};
use crate::color::Color;
use crate::ray::Ray;
use crate::sampler::get_disk_sampler;
use crate::world::World;

pub struct ThinLensCamera {
    pub lens_radius: f64,
    pub focal_plane_distance: f64, // f
    pub setting: CameraSetting,
}

impl Camera for ThinLensCamera {
    fn render_scene(&self, world: &World) -> Vec<Color> {
        let hres = world.vp.hres;
        let vres = world.vp.vres;
        let pixel_size = world.vp.pixel_size;
        (0..(world.vp.hres * world.vp.vres))
            .into_par_iter()
            .map(|n| {
                let (i, j) = (n % hres, n / hres);
                get_disk_sampler(self.setting.sample_points_sqrt)
                    .map(|(sp, dp)| {
                        let p = sp
                            .add(Vector2::new(
                                i as f64 - hres as f64 / 2.0,
                                j as f64 - vres as f64 / 2.0,
                            ))
                            .mul(pixel_size);
                        let ray = self.get_ray(
                            p.add(sp.to_vector()),
                            Point2::new(dp.x * self.lens_radius, dp.y * self.lens_radius),
                        );
                        world.trace(&ray, 0)
                    })
                    .fold(Vec3::zero(), |v1, v2| v1.add(v2))
                    .div((self.setting.sample_points_sqrt * self.setting.sample_points_sqrt) as f64)
            })
            .collect()
    }
}

impl ThinLensCamera {
    fn get_ray(&self, p: Point2<f64>, lens_point: Point2<f64>) -> Ray {
        let origin = self
            .setting
            .eye
            .add(self.setting.u.mul(lens_point.x))
            .add(self.setting.v.mul(lens_point.y));
        let dp = p
            .mul(self.focal_plane_distance)
            .div(self.setting.view_plane_distance)
            .sub(lens_point);
        let dir = self
            .setting
            .u
            .mul(dp.x)
            .add(self.setting.v.mul(dp.y))
            .sub(self.setting.w.mul(self.focal_plane_distance))
            .normalize();
        Ray::new(origin, dir)
    }
}
