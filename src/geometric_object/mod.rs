//! Geometric objects and primitives for ray tracing.
//!
//! This module provides the core geometric shapes that can be rendered,
//! including spheres, triangles, and meshes. All geometric objects implement
//! the `Geometry` trait for ray intersection testing.

use nalgebra::Point3;

mod sphere;
mod triangle;

pub use sphere::*;
pub use triangle::*;

use crate::{
    aabb::Aabb,
    model::Vec3,
    ray::{HitRecord, Ray},
    sampler::Sampler,
};

/// Trait for all geometric objects that can be ray-traced.
///
/// This trait defines the interface for ray-object intersection testing
/// and provides methods for bounding box computation and sampling.
pub trait Geometry: Send + Sync {
    /// Scales the geometry by the given factor.
    ///
    /// # Arguments
    /// * `l` - Scale factor to apply
    fn scale(&mut self, _l: f64) {}

    /// Tests for ray-object intersection within the given range.
    ///
    /// # Arguments
    /// * `ray` - The ray to test for intersection
    /// * `t_min` - Minimum valid distance along the ray
    /// * `t_max` - Maximum valid distance along the ray
    ///
    /// # Returns
    /// `Some(HitRecord)` if the ray intersects the object, `None` otherwise
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>>;

    /// Computes the surface normal at the given point.
    ///
    /// # Arguments
    /// * `p` - Point on the surface
    ///
    /// # Returns
    /// Normalized surface normal vector
    fn normal(&self, _p: &Point3<f64>) -> Vec3 {
        Vec3::zeros()
    }

    /// Returns the geometric center of the object.
    fn get_center(&self) -> Point3<f64> {
        Point3::origin()
    }

    /// Returns the minimum corner of the axis-aligned bounding box.
    fn get_min_point(&self) -> Point3<f64>;

    /// Returns the maximum corner of the axis-aligned bounding box.
    fn get_max_point(&self) -> Point3<f64>;

    /// Computes the axis-aligned bounding box for the object.
    ///
    /// # Returns
    /// An AABB containing the entire geometry
    fn get_bounding_box(&self) -> Aabb {
        Aabb::new(self.get_min_point(), self.get_max_point())
    }

    /// Generates sample points on the object's surface.
    ///
    /// Used for area light sampling and other Monte Carlo techniques.
    ///
    /// # Arguments
    /// * `sampler` - The sampler to use for generating random points
    ///
    /// # Returns
    /// A vector of sample points on the surface
    fn get_samples(&self, _sampler: &Sampler) -> Vec<Point3<f64>> {
        vec![]
    }
}
