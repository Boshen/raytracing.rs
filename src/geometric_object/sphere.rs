use nalgebra::Point3;
use std::ops::{MulAssign, SubAssign};

use crate::aabb::Aabb;
use crate::geometric_object::Geometry;
use crate::model::Vec3;
use crate::ray::{HitRecord, Ray};

pub struct Sphere {
    radius: f64,
    center: Point3<f64>,
    material_id: usize,
}

impl Sphere {
    pub fn new(material_id: usize, radius: f64, center: Point3<f64>, scale: f64) -> Self {
        let mut sphere = Self {
            radius,
            center,
            material_id,
        };
        sphere.scale(scale);
        sphere
    }
}

impl Geometry for Sphere {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let radius = self.radius;
        let center = Vec3::new(self.center.x, self.center.y, self.center.z);
        let start = Vec3::new(ray.origin.x, ray.origin.y, ray.origin.z);
        let dir = ray.dir;

        let a = dir.dot(&dir);
        let b = 2.0 * dir.dot(&(start - center));
        let c =
            center.dot(&center) + start.dot(&start) - 2.0 * center.dot(&start) - radius * radius;

        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            return None;
        }

        let t = (-b - disc.sqrt()) / (2.0 * a);
        if t < 0.0 {
            return None;
        }
        if t < t_min || t > t_max {
            return None;
        }

        let hit_point = ray.get_point(t);
        Some(HitRecord {
            dist: t,
            hit_point,
            normal: self.normal(&hit_point),
            material_id: self.material_id,
        })
    }

    fn scale(&mut self, l: f64) {
        self.center.mul_assign(2.0 / l);
        self.center.sub_assign(Vec3::repeat(1.0));
        self.center.mul_assign(-1.0);
        self.radius = (self.radius * 2.0) / l;
    }

    fn normal(&self, p: &Point3<f64>) -> Vec3 {
        ((p - self.center) / self.radius).normalize()
    }

    fn get_center(&self) -> Point3<f64> {
        self.center
    }

    fn get_min_point(&self) -> Point3<f64> {
        self.center - Vec3::repeat(self.radius)
    }

    fn get_max_point(&self) -> Point3<f64> {
        self.center + Vec3::repeat(self.radius)
    }

    fn get_bounding_box(&self) -> Aabb {
        Aabb::new(self.get_min_point(), self.get_max_point())
    }

    fn get_samples(&self, _sample_points_sqrt: u8) -> Vec<Point3<f64>> {
        vec![]
    }

    fn get_material_id(&self) -> usize {
        self.material_id
    }
}
