use nalgebra::{Cross, Dot, Norm};
use std::ops::{Mul, Sub};

use crate::model::Vec3;
use crate::ray::Ray;

use crate::hittable::Hittable;

pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

impl Hittable for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let epsilon = 0.000001;
        let e1 = self.1.sub(self.0);
        let e2 = self.2.sub(self.0);

        let h = ray.dir.cross(&e2);
        let a = e1.dot(&h);
        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin.sub(self.0);
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

        return Some(t);
    }

    fn normal(&self, _p: &Vec3) -> Vec3 {
        let e1 = self.1.sub(self.0);
        let e2 = self.2.sub(self.0);
        return e2.cross(&e1).normalize();
    }

    fn get_min_point(&self) -> Vec3 {
        return Vec3::new(
            self.0.x.min(self.1.x).min(self.2.x),
            self.0.y.min(self.1.y).min(self.2.y),
            self.0.z.min(self.1.z).min(self.2.z),
        );
    }

    fn get_max_point(&self) -> Vec3 {
        return Vec3::new(
            self.0.x.max(self.1.x).max(self.2.x),
            self.0.y.max(self.1.y).max(self.2.y),
            self.0.z.max(self.1.z).max(self.2.z),
        );
    }

    fn scale(&mut self, l: f64) {
        self.0 = self.0.mul(2.0 / l);
        self.1 = self.1.mul(2.0 / l);
        self.2 = self.2.mul(2.0 / l);

        self.0 = self.0.sub(Vec3::new(1.0, 1.0, 1.0));
        self.1 = self.1.sub(Vec3::new(1.0, 1.0, 1.0));
        self.2 = self.2.sub(Vec3::new(1.0, 1.0, 1.0));

        self.0.x = -self.0.x;
        self.1.x = -self.1.x;
        self.2.x = -self.2.x;

        self.0.y = -self.0.y;
        self.1.y = -self.1.y;
        self.2.y = -self.2.y;
    }
}
