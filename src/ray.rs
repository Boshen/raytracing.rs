//! Ray representation and hit testing for the raytracer.
//!
//! This module defines the core ray structure and hit record types
//! used throughout the ray tracing process.

use nalgebra::Point3;

use crate::{material::Material, model::Vec3, renderer::Renderer};

/// Represents a ray in 3D space with an origin and direction.
///
/// Rays are the fundamental primitive in ray tracing, representing
/// the path of light through the scene.
pub struct Ray {
    /// The starting point of the ray
    pub origin: Point3<f64>,
    /// The normalized direction vector
    pub dir: Vec3,
}

impl Ray {
    /// Creates a new ray with the specified origin and direction.
    ///
    /// # Arguments
    /// * `origin` - The starting point of the ray
    /// * `dir` - The direction vector (should be normalized)
    #[must_use]
    pub fn new(origin: Point3<f64>, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    /// Computes a point along the ray at the specified distance.
    ///
    /// # Arguments
    /// * `distance` - The distance from the origin along the ray direction
    ///
    /// # Returns
    /// The 3D point at `origin + direction * distance`
    #[must_use]
    pub fn get_point(&self, distance: f64) -> Point3<f64> {
        self.origin + self.dir * distance
    }
}

/// Records information about a ray-object intersection.
///
/// This lightweight structure stores essential hit information
/// for initial intersection testing.
pub struct HitRecord<'a> {
    /// Distance from ray origin to hit point
    pub dist: f64,
    /// The 3D point where the ray intersects the object
    pub hit_point: Point3<f64>,
    /// Surface normal at the hit point (normalized)
    pub normal: Vec3,
    /// Reference to the material at the hit point
    pub material: &'a dyn Material,
}

/// Complete hit information for shading calculations.
///
/// Contains all necessary data for computing the color contribution
/// at an intersection point, including recursive ray tracing context.
pub struct Hit<'a> {
    /// The ray that caused this hit
    pub ray: &'a Ray,
    /// The 3D point where the ray intersects the object
    pub hit_point: Point3<f64>,
    /// Surface normal at the hit point (normalized)
    pub normal: Vec3,
    /// Reference to the renderer for recursive ray tracing
    pub renderer: &'a Renderer,
    /// Current recursion depth for ray bounces
    pub depth: u8,
    /// Reference to the material at the hit point
    pub material: &'a dyn Material,
}
