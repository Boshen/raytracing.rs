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
        // Optimized ray-sphere intersection using improved quadratic formula
        // Avoids unnecessary conversions and leverages FMA instructions

        // Vector from ray origin to sphere center
        let oc = ray.origin - self.center;

        // Coefficients for quadratic equation
        // Using half_b optimization to reduce operations
        let a = ray.dir.dot(&ray.dir); // Usually 1.0 for normalized rays
        let half_b = oc.dot(&ray.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;

        // Calculate discriminant with FMA: half_b² - ac
        let discriminant = half_b.mul_add(half_b, -a * c);

        // Early exit if no intersection
        if discriminant < 0.0 {
            return None;
        }

        // Calculate nearest intersection point
        let sqrt_disc = discriminant.sqrt();

        // Try nearer intersection first
        let t = (-half_b - sqrt_disc) / a;

        // Check if nearer intersection is valid
        if t < t_min || t > t_max {
            // Try farther intersection
            let t_far = (-half_b + sqrt_disc) / a;
            if t_far < t_min || t_far > t_max {
                return None;
            }
            // Use farther intersection
            let hit_point = ray.get_point(t_far);
            return Some(HitRecord {
                dist: t_far,
                hit_point,
                normal: self.normal(&hit_point),
                material: &self.material,
            });
        }

        // Use nearer intersection
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
