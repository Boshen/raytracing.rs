use nalgebra::Norm;
use std::ops::{Add, Mul, Sub};

use crate::model::Vec3;
use crate::ray::Ray;

use crate::geometric_object::GeometricObject;

#[derive(Copy, Clone)]
pub struct Sphere {
    radius: f64,
    center: Vec3,
}

impl Sphere {
    pub fn new(radius: f64, center: Vec3, scale: f64) -> Sphere {
        let mut sphere = Sphere { radius, center };
        sphere.scale(scale);
        sphere
    }
}

impl GeometricObject for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let center = self.center;
        let radius = self.radius;
        let start = ray.origin;
        let dx = ray.dir.x;
        let dy = ray.dir.y;
        let dz = ray.dir.z;

        let a = dx * dx + dy * dy + dz * dz;
        let b = 2.0 * dx * (start.x - center.x)
            + 2.0 * dy * (start.y - center.y)
            + 2.0 * dz * (start.z - center.z);
        let c = center.x * center.x
            + center.y * center.y
            + center.z * center.z
            + start.x * start.x
            + start.y * start.y
            + start.z * start.z
            - 2.0 * (center.x * start.x + center.y * start.y + center.z * start.z)
            - radius * radius;

        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            return None;
        }

        let t = (-b - disc.sqrt()) / (2.0 * a);
        if t < 0.0 {
            return None;
        }

        Some(t)
    }

    fn normal(&self, p: &Vec3) -> Vec3 {
        p.sub(self.center).mul(1.0 / self.radius).normalize()
    }

    fn get_center(&self) -> Vec3 {
        self.center
    }

    fn get_min_point(&self) -> Vec3 {
        self.center.sub(self.radius)
    }

    fn get_max_point(&self) -> Vec3 {
        self.center.add(self.radius)
    }

    fn get_samples(&self, _sample_points_sqrt: u32) -> Vec<Vec3> {
        vec![]
    }

    fn scale(&mut self, l: f64) {
        self.center = self.center.mul(2.0 / l);
        self.center = self.center.sub(Vec3::new(1.0, 1.0, 1.0));
        self.center.x = -self.center.x;
        self.center.y = -self.center.y;
        self.radius = (self.radius * 2.0) / l;
    }
}
