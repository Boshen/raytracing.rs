use nalgebra::{center, Point3};

use crate::aabb::AABB;
use crate::geometric_object::Geometry;
use crate::model::Vec3;
use crate::ray::{HitRecord, Ray};
use crate::sampler::get_triangle_sampler;

pub struct Triangle {
    pub x: Point3<f64>,
    pub y: Point3<f64>,
    pub z: Point3<f64>,
    material_id: usize,
}

impl Triangle {
    pub fn new(
        material_id: usize,
        x: Point3<f64>,
        y: Point3<f64>,
        z: Point3<f64>,
        scale: f64,
    ) -> Triangle {
        let mut triangle = Triangle {
            x,
            y,
            z,
            material_id,
        };
        triangle.scale(scale);
        triangle
    }
}

impl Geometry for Triangle {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let epsilon = 0.000_001;
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
        if !(0.0..=1.0).contains(&u) {
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

    fn get_bounding_box(&self) -> AABB {
        AABB::new(self.get_min_point(), self.get_max_point())
    }

    fn get_samples(&self, sample_points_sqrt: usize) -> Vec<Point3<f64>> {
        get_triangle_sampler(sample_points_sqrt, self).collect()
    }

    fn get_material_id(&self) -> usize {
        self.material_id
    }
}
