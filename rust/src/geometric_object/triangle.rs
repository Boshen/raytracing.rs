use nalgebra::{Cross, Dot, Norm, Point3};

use crate::aabb::AABB;
use crate::geometric_object::GeometricObject;
use crate::model::Vec3;
use crate::ray::{HitRecord, Ray};
use crate::sampler::get_triangle_sampler;

#[derive(Clone)]
pub struct Triangle {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
    material_id: usize,
}

impl Triangle {
    pub fn new(material_id: usize, x: Vec3, y: Vec3, z: Vec3, scale: f64) -> Triangle {
        let mut triangle = Triangle {
            material_id,
            x,
            y,
            z,
        };
        triangle.scale(scale);
        triangle
    }
}

impl GeometricObject for Triangle {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let epsilon = 0.000001;
        let e1 = self.y - self.x;
        let e2 = self.z - self.x;

        let h = ray.dir.cross(&e2);
        let a = e1.dot(&h);
        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = a.recip();
        let s = ray.origin - self.x;
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
        self.x = self.x * (2.0 / l);
        self.y = self.y * (2.0 / l);
        self.z = self.z * (2.0 / l);

        self.x = self.x - Vec3::new(1.0, 1.0, 1.0);
        self.y = self.y - Vec3::new(1.0, 1.0, 1.0);
        self.z = self.z - Vec3::new(1.0, 1.0, 1.0);

        self.x.x = -self.x.x;
        self.y.x = -self.y.x;
        self.z.x = -self.z.x;

        self.x.y = -self.x.y;
        self.y.y = -self.y.y;
        self.z.y = -self.z.y;
    }

    fn normal(&self, _p: &Vec3) -> Vec3 {
        let e1 = self.y - self.x;
        let e2 = self.z - self.x;
        e2.cross(&e1).normalize()
    }

    fn get_center(&self) -> Vec3 {
        (self.x + self.y + self.z) / 3.0
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

    fn get_bounding_box(&self) -> AABB {
        AABB::new(self.get_min_point(), self.get_max_point())
    }

    fn get_samples(&self, sample_points_sqrt: usize) -> Vec<Vec3> {
        get_triangle_sampler(sample_points_sqrt, &self).collect()
    }

    fn get_material_id(&self) -> usize {
        self.material_id
    }
}
