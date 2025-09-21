use nalgebra::{Point3, center};

use super::Geometry;
use crate::{
    material::Material,
    model::Vec3,
    ray::{HitRecord, Ray},
    sampler::Sampler,
};

pub struct Triangle<M: Material> {
    pub x: Point3<f64>,
    pub y: Point3<f64>,
    pub z: Point3<f64>,
    material: M,
}

impl<M: Material> Triangle<M> {
    pub fn new(material: M, x: Point3<f64>, y: Point3<f64>, z: Point3<f64>, scale: f64) -> Self {
        let mut triangle = Self { x, y, z, material };
        triangle.scale(scale);
        triangle
    }
}

impl<M: Material> Geometry for Triangle<M> {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        // Optimized Möller-Trumbore intersection algorithm
        // Reduces branching and leverages FMA instructions

        const EPSILON: f64 = 1e-8;

        // Calculate edges and determinant vector
        let edge1 = self.y - self.x;
        let edge2 = self.z - self.x;
        let pvec = ray.dir.cross(&edge2);
        let det = edge1.dot(&pvec);

        // Early exit if ray is parallel to triangle (determinant near zero)
        // Using abs for two-sided intersection testing
        if det.abs() < EPSILON {
            return None;
        }

        let inv_det = det.recip();
        let tvec = ray.origin - self.x;

        // Calculate u parameter and test bounds
        let u = tvec.dot(&pvec) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        // Calculate v parameter and test bounds
        let qvec = tvec.cross(&edge1);
        let v = ray.dir.dot(&qvec) * inv_det;

        // Combined test for v and (u+v) to reduce branching
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Calculate t parameter (distance along ray)
        let t = edge2.dot(&qvec) * inv_det;

        // Check if intersection is within valid range
        if t < t_min || t > t_max {
            return None;
        }

        // Valid intersection found
        let hit_point = ray.get_point(t);
        Some(HitRecord {
            dist: t,
            hit_point,
            normal: self.normal(&hit_point),
            material: &self.material,
        })
    }

    fn scale(&mut self, l: f64) {
        self.x *= 2.0 / l;
        self.y *= 2.0 / l;
        self.z *= 2.0 / l;

        self.x -= Vec3::new(1.0, 1.0, 1.0);
        self.y -= Vec3::new(1.0, 1.0, 1.0);
        self.z -= Vec3::new(1.0, 1.0, 1.0);

        self.x.x = -self.x.x;
        self.y.x = -self.y.x;
        self.z.x = -self.z.x;

        self.x.y = -self.x.y;
        self.y.y = -self.y.y;
        self.z.y = -self.z.y;
    }

    fn normal(&self, _p: &Point3<f64>) -> Vec3 {
        let e1 = self.y - self.x;
        let e2 = self.z - self.x;
        e2.cross(&e1).normalize()
    }

    fn get_center(&self) -> Point3<f64> {
        center(&center(&self.x, &self.y), &self.z)
    }

    fn get_min_point(&self) -> Point3<f64> {
        Point3::new(
            self.x.x.min(self.y.x).min(self.z.x),
            self.x.y.min(self.y.y).min(self.z.y),
            self.x.z.min(self.y.z).min(self.z.z),
        )
    }

    fn get_max_point(&self) -> Point3<f64> {
        Point3::new(
            self.x.x.max(self.y.x).max(self.z.x),
            self.x.y.max(self.y.y).max(self.z.y),
            self.x.z.max(self.y.z).max(self.z.z),
        )
    }

    fn get_samples(&self, sampler: &Sampler) -> Vec<Point3<f64>> {
        sampler.triangle(&self.x, &self.y, &self.z).collect()
    }
}
