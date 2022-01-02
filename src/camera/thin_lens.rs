use nalgebra::{Point2, Vector2};
use rayon::prelude::*;

use super::{Camera, Setting};
use crate::color::Color;
use crate::ray::Ray;
use crate::sampler::get_disk;
use crate::world::World;

pub struct ThinLens {
    setting: Setting,
    lens_radius: f64,
    focal_plane_distance: f64, // f
}

impl ThinLens {
    #[must_use]
    pub const fn new(setting: Setting, lens_radius: f64, focal_plane_distance: f64) -> Self {
        Self {
            setting,
            lens_radius,
            focal_plane_distance,
        }
    }

    fn get_ray(&self, p: Point2<f64>, lens_point: Point2<f64>) -> Ray {
        let origin =
            self.setting.eye + self.setting.u * lens_point.x + self.setting.v * lens_point.y;
        let dp = p * self.focal_plane_distance / self.setting.view_plane_distance - lens_point;
        let dir = (self.setting.u * dp.x + self.setting.v * dp.y
            - self.setting.w * self.focal_plane_distance)
            .normalize();
        Ray::new(origin, dir)
    }
}

impl Camera for ThinLens {
    fn render_scene(&self, world: &World) -> Vec<Color> {
        let hres = world.vp.hres;
        let vres = world.vp.vres;
        let pixel_size = world.vp.pixel_size;
        let sample_points = self.setting.sample_points_sqrt.pow(2);

        let vec = (0..(hres * vres))
            .into_par_iter()
            .flat_map_iter(|n| {
                let i = f64::from(n % hres) - f64::from(hres) / 2.0;
                let j = f64::from(n / hres) - f64::from(vres) / 2.0;
                get_disk(self.setting.sample_points_sqrt).map(move |(sp, dp)| {
                    let start_point = (sp + Vector2::new(i + sp.x, j + sp.y)) * pixel_size;
                    let end_point = dp * self.lens_radius;
                    self.get_ray(start_point, end_point)
                })
            })
            .map(|ray| world.trace(&ray, 0))
            .collect::<Vec<_>>();

        vec.chunks(sample_points.into())
            .map(|chunks| chunks.iter().sum::<Color>() / sample_points as f64)
            .collect()
    }
}
