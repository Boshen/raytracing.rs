use nalgebra::{Cross, Dot, Norm, Vector3};
use std::ops::{Add, Mul, Sub};

use crate::aabb::AABB;
use crate::ray::Ray;

pub type Color = Vector3<f64>;
pub type Vec3 = Vector3<f64>;

pub const SAMPLE_POINTS: u32 = 10;

pub struct Model {
    pub material: Material,
    pub hittables: Vec<Box<dyn Hittable>>,
    pub aabb: AABB,
}

#[derive(Copy, Clone)]
pub struct Material {
    pub diffuse_reflection: f64,
    pub diffuse_color: Color,
    pub reflection: f64,
    pub specular_refection: f64,
    pub shininess: f64,
    pub transparent: bool,
}

pub trait Hittable: Send + Sync {
    fn scale(&mut self, l: f64) -> ();
    fn intersects(&self, ray: &Ray) -> Option<f64>;
    fn normal(&self, p: &Vec3) -> Vec3;
    fn get_min_point(&self) -> Vec3;
    fn get_max_point(&self) -> Vec3;
}

pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
}

impl Model {
    pub fn new(l: f64, material: Material, hittables: Vec<Box<dyn Hittable>>) -> Model {
        let mut model = Model {
            material: material,
            hittables: hittables,
            aabb: AABB::new(vec![], vec![]),
        };
        model.scale(l);
        return model;
    }

    fn scale(&mut self, l: f64) {
        for h in &mut self.hittables {
            h.scale(l);
        }
        let mins = self.hittables.iter().map(|h| h.get_min_point()).collect();
        let maxs = self.hittables.iter().map(|h| h.get_max_point()).collect();
        self.aabb = AABB::new(mins, maxs)
    }
}

impl Hittable for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let epsilon = 0.000001;
        let e1 = self.1.sub(self.0);
        let e2 = self.2.sub(self.0);

        let h = ray.direction.cross(&e2);
        let a = e1.dot(&h);
        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.start.sub(self.0);
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&e1);
        let v = f * ray.direction.dot(&q);
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

impl Sphere {
    pub fn new(radius: f64, center: Vec3) -> Sphere {
        Sphere {
            radius: radius,
            center: center,
        }
    }
}

impl Hittable for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let center = self.center;
        let radius = self.radius;
        let start = ray.start;
        let dx = ray.direction.x;
        let dy = ray.direction.y;
        let dz = ray.direction.z;

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

        return Some(t);
    }

    fn normal(&self, p: &Vec3) -> Vec3 {
        return p.sub(self.center).mul(1.0 / self.radius).normalize();
    }

    fn get_min_point(&self) -> Vec3 {
        return self.center.sub(self.radius);
    }

    fn get_max_point(&self) -> Vec3 {
        return self.center.add(self.radius);
    }

    fn scale(&mut self, l: f64) {
        self.center = self.center.mul(2.0 / l);
        self.center = self.center.sub(Vec3::new(1.0, 1.0, 1.0));
        self.center.x = -self.center.x;
        self.center.y = -self.center.y;
        self.radius = (self.radius * 2.0) / l;
    }
}
