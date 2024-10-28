use nalgebra::{Point2, Vector2};

use super::{Camera, Setting};
use crate::{ray::Ray, sampler::Sampler};

pub struct ThinLens {
    setting: Setting,
    lens_radius: f64,
    focal_plane_distance: f64, // f
}

impl ThinLens {
    #[must_use]
    pub const fn new(setting: Setting, lens_radius: f64, focal_plane_distance: f64) -> Self {
        Self { setting, lens_radius, focal_plane_distance }
    }

    #[must_use]
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
    fn setting(&self) -> &Setting {
        &self.setting
    }

    #[must_use]
    fn get_rays(&self, origin: Point2<f64>, sampler: &Sampler) -> Vec<Ray> {
        sampler
            .disk()
            .map(move |(sp, dp)| {
                let start_point = sp + Vector2::new(origin.x + sp.x, origin.y + sp.y);
                let end_point = dp * self.lens_radius;
                self.get_ray(start_point, end_point)
            })
            .collect()
    }
}
