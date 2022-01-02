use nalgebra::{Point2, Vector2};

use super::{Camera, Setting};
use crate::ray::Ray;
use crate::sampler::get_square;

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
    fn setting(&self) -> &Setting {
        &self.setting
    }

    #[must_use]
    fn get_rays(&self, origin: Point2<f64>) -> Vec<Ray> {
        get_square(self.setting.sample_points_sqrt)
            .map(|dp| self.get_ray(origin - dp))
            .collect()
    }
}
