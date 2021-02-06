use crate::model::Vec3;
use nalgebra::{Cross, Norm, Point2, Vector2};
use num_traits::identities::Zero;
use rayon::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

use crate::camera::Camera;
use crate::color::Color;
use crate::ray::Ray;
use crate::sampler::get_disk_sampler;
use crate::world::World;

pub struct ThinLensCamera {
    lens_radius: f64,
    focal_plane_distance: f64, // f

    sample_points_sqrt: u32,
    up: Vec3,
    eye: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    view_plane_distance: f64, // d
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
                let x = pixel_size * (i as f64 - (hres as f64) / 2.0);
                let y = pixel_size * (j as f64 - (vres as f64) / 2.0);
                self.antialias(world, Point2::new(x, y))
            })
            .collect()
    }
}

impl ThinLensCamera {
    pub fn new(eye: Vec3, lookat: Vec3, view_plane_distance: f64) -> ThinLensCamera {
        let up = Vec3::new(0.0, 1.0, 0.0);
        let w = eye.sub(lookat).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u).normalize();
        ThinLensCamera {
            lens_radius: 0.01,
            focal_plane_distance: 1000.0,

            eye,
            w,
            u,
            v,
            up,
            view_plane_distance,
            sample_points_sqrt: 10,
        }
    }

    fn antialias(&self, world: &World, p: Point2<f64>) -> Color {
        get_disk_sampler(self.sample_points_sqrt)
            .map(|(spx, spy, dpx, dpy)| {
                world.trace(
                    &self.get_ray(
                        p.add(Vector2::new(spx, spy)),
                        Point2::new(dpx * self.lens_radius, dpy * self.lens_radius),
                    ),
                    0,
                )
            })
            .fold(Vec3::zero(), |v1, v2| v1.add(v2))
            .div(self.sample_points_sqrt as f64 * self.sample_points_sqrt as f64)
    }

    fn get_ray(&self, p: Point2<f64>, lens_point: Point2<f64>) -> Ray {
        let origin = self
            .eye
            .add(self.u.mul(lens_point.x))
            .add(self.v.mul(lens_point.y));
        let dp = p
            .mul(self.focal_plane_distance)
            .div(self.view_plane_distance)
            .sub(lens_point);
        let dir = self
            .u
            .mul(dp.x)
            .add(self.v.mul(dp.y))
            .sub(self.w.mul(self.focal_plane_distance))
            .normalize();
        Ray::new(origin, dir)
    }
}
