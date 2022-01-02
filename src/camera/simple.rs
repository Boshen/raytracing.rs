use nalgebra::{Point2, Vector2};
use rayon::prelude::*;

use super::{Camera, Setting};
use crate::color::Color;
use crate::ray::Ray;
use crate::sampler::get_square;
use crate::world::World;

pub struct Simple {
    setting: Setting,
}

impl Simple {
    #[must_use]
    pub const fn new(setting: Setting) -> Self {
        Self { setting }
    }

    #[must_use]
    fn get_ray(&self, dir: Vector2<f64>) -> Ray {
        let dir = (self.setting.u * dir.x + self.setting.v * dir.y
            - self.setting.w * self.setting.view_plane_distance)
            .normalize();
        Ray::new(self.setting.eye, dir)
    }
}

impl Camera for Simple {
    fn render_scene(&self, world: &World) -> Vec<Color> {
        let hres = self.setting.view_width;
        let vres = self.setting.view_height;
        let pixel_size = self.setting.pixel_size;
        let sample_points = self.setting.sample_points_sqrt.pow(2);

        (0..(hres * vres))
            .into_par_iter()
            .map(|n| {
                let (i, j) = (n % hres, n / hres);
                let p = Point2::new(
                    pixel_size * (i as f64 - (hres as f64) / 2.0),
                    pixel_size * (j as f64 - (vres as f64) / 2.0),
                );
                get_square(self.setting.sample_points_sqrt)
                    .map(|dp| {
                        let ray = self.get_ray(p - dp);
                        world.trace(&ray, 0)
                    })
                    .sum::<Color>()
                    / (sample_points as f64)
            })
            .collect()
    }
}
