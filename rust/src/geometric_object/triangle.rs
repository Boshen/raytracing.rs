use crate::sampler::get_triangle_sampler;
use nalgebra::{Cross, Dot, Norm};

use crate::model::Vec3;
use crate::ray::Ray;

use crate::geometric_object::GeometricObject;

#[derive(Copy, Clone)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

impl Triangle {
    pub fn new(x: Vec3, y: Vec3, z: Vec3, scale: f64) -> Triangle {
        let mut triangle = Triangle(x, y, z);
        triangle.scale(scale);
        triangle
    }
}

impl GeometricObject for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let epsilon = 0.000001;
        let e1 = self.1 - self.0;
        let e2 = self.2 - self.0;

        let h = ray.dir.cross(&e2);
        let a = e1.dot(&h);
        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = a.recip();
        let s = ray.origin - self.0;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&e1);
        let v = f * ray.dir.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * e2.dot(&q);
        if t <= epsilon {
            return None;
        }

        Some(t)
    }

    fn normal(&self, _p: &Vec3) -> Vec3 {
        let e1 = self.1 - self.0;
        let e2 = self.2 - self.0;
        e2.cross(&e1).normalize()
    }

    fn get_center(&self) -> Vec3 {
        (self.0 + self.1 + self.2) / 3.0
    }

    fn get_min_point(&self) -> Vec3 {
        Vec3::new(
            self.0.x.min(self.1.x).min(self.2.x),
            self.0.y.min(self.1.y).min(self.2.y),
            self.0.z.min(self.1.z).min(self.2.z),
        )
    }

    fn get_max_point(&self) -> Vec3 {
        Vec3::new(
            self.0.x.max(self.1.x).max(self.2.x),
            self.0.y.max(self.1.y).max(self.2.y),
            self.0.z.max(self.1.z).max(self.2.z),
        )
    }

    fn get_samples(&self, sample_points_sqrt: usize) -> Vec<Vec3> {
        get_triangle_sampler(sample_points_sqrt, &self).collect()
    }

    fn scale(&mut self, l: f64) {
        self.0 = self.0 * (2.0 / l);
        self.1 = self.1 * (2.0 / l);
        self.2 = self.2 * (2.0 / l);

        self.0 = self.0 - Vec3::new(1.0, 1.0, 1.0);
        self.1 = self.1 - Vec3::new(1.0, 1.0, 1.0);
        self.2 = self.2 - Vec3::new(1.0, 1.0, 1.0);

        self.0.x = -self.0.x;
        self.1.x = -self.1.x;
        self.2.x = -self.2.x;

        self.0.y = -self.0.y;
        self.1.y = -self.1.y;
        self.2.y = -self.2.y;
    }
}
