use nalgebra::{Point2, Vector2};

use super::{Camera, Setting};
use crate::ray::Ray;
use crate::sampler::Sampler;

pub struct Pinhole {
    setting: Setting,
}

impl Pinhole {
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

impl Camera for Pinhole {
    fn setting(&self) -> &Setting {
        &self.setting
    }

    #[must_use]
    fn get_rays(&self, origin: Point2<f64>, sampler: &Sampler) -> Vec<Ray> {
        sampler
            .square()
            .map(|dp| self.get_ray(origin - dp))
            .collect()
    }
}
