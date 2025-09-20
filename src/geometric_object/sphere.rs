//! Sphere geometric primitive.

use std::ops::{MulAssign, SubAssign};

use nalgebra::Point3;

use crate::{
    geometric_object::Geometry,
    material::Material,
    model::Vec3,
    ray::{HitRecord, Ray},
};

/// A sphere defined by its center and radius.
///
/// Spheres are one of the most efficient primitives for ray tracing
/// due to their simple intersection calculations.
pub struct Sphere<M: Material> {
    /// Sphere radius
    radius: f64,
    /// Sphere center in world space
    center: Point3<f64>,
    /// Material properties
    material: M,
}

impl<M: Material> Sphere<M> {
    /// Creates a new sphere with the given material, radius, and center.
    ///
    /// # Arguments
    /// * `material` - The material properties for shading
    /// * `radius` - The sphere's radius
    /// * `center` - The sphere's center position
    /// * `scale` - Scale factor to apply to the sphere
    pub fn new(material: M, radius: f64, center: Point3<f64>, scale: f64) -> Self {
        let mut sphere = Self { radius, center, material };
        sphere.scale(scale);
        sphere
    }
}

impl<M: Material> Geometry for Sphere<M> {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        // Extract sphere properties
        let radius = self.radius;
        let center = Vec3::new(self.center.x, self.center.y, self.center.z);
        let start = Vec3::new(ray.origin.x, ray.origin.y, ray.origin.z);
        let dir = ray.dir;

        // Solve quadratic equation for ray-sphere intersection:
        // |P(t) - C|² = r²  where P(t) = O + t*D
        // Expands to: at² + bt + c = 0
        let a = dir.dot(&dir); // a = D·D (usually 1 if normalized)
        let b = 2.0 * dir.dot(&(start - center)); // b = 2D·(O-C)
        let c = radius.mul_add(
            // c = (O-C)·(O-C) - r²
            -radius,
            2.0f64.mul_add(-center.dot(&start), center.dot(&center) + start.dot(&start)),
        );

        // Calculate discriminant: b² - 4ac
        let disc = b.mul_add(b, -(4.0 * a * c));

        // No intersection if discriminant is negative
        if disc < 0.0 {
            return None;
        }

        // Find the nearest intersection point (smaller t value)
        let t = (-b - disc.sqrt()) / (2.0 * a);

        // Check if intersection is behind ray origin
        if t < 0.0 {
            return None;
        }

        // Check if intersection is within valid range
        if t < t_min || t > t_max {
            return None;
        }

        // Calculate hit point and return hit record
        let hit_point = ray.get_point(t);
        Some(HitRecord {
            dist: t,
            hit_point,
            normal: self.normal(&hit_point),
            material: &self.material,
        })
    }

    fn scale(&mut self, l: f64) {
        // Transform from model space to world space
        // This converts from [-1, 1] normalized coordinates to scene units
        self.center.mul_assign(2.0 / l);
        self.center.sub_assign(Vec3::repeat(1.0));
        self.center.mul_assign(-1.0);
        self.radius = (self.radius * 2.0) / l;
    }

    fn normal(&self, p: &Point3<f64>) -> Vec3 {
        // Surface normal at point p is the normalized vector from center to p
        ((p - self.center) / self.radius).normalize()
    }

    fn get_center(&self) -> Point3<f64> {
        self.center
    }

    fn get_min_point(&self) -> Point3<f64> {
        // Bounding box minimum: center minus radius in all dimensions
        self.center - Vec3::repeat(self.radius)
    }

    fn get_max_point(&self) -> Point3<f64> {
        // Bounding box maximum: center plus radius in all dimensions
        self.center + Vec3::repeat(self.radius)
    }
}
