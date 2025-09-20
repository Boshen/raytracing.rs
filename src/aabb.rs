//! Axis-Aligned Bounding Box (AABB) for acceleration structures.
//!
//! AABBs are rectangular boxes aligned with the coordinate axes,
//! used to quickly reject rays that don't intersect complex geometry.

use nalgebra::Point3;

use crate::ray::Ray;

/// An axis-aligned bounding box defined by minimum and maximum corners.
///
/// AABBs are used in BVH nodes to quickly test whether a ray might
/// intersect the geometry contained within.
pub struct Aabb {
    /// Minimum corner of the bounding box (smallest x, y, z values)
    pub min: Point3<f64>,
    /// Maximum corner of the bounding box (largest x, y, z values)
    pub max: Point3<f64>,
}

impl Aabb {
    /// Creates a new AABB from minimum and maximum corners.
    #[must_use]
    pub const fn new(min: Point3<f64>, max: Point3<f64>) -> Self {
        Self { min, max }
    }

    /// Tests if a ray intersects this bounding box using the slab method.
    ///
    /// The algorithm works by treating the AABB as the intersection of three
    /// pairs of parallel planes (slabs). For each dimension:
    /// 1. Calculate where the ray intersects the min and max planes
    /// 2. Update the valid intersection interval
    /// 3. If the interval becomes invalid, there's no intersection
    ///
    /// # Arguments
    /// * `r` - The ray to test
    /// * `tmin` - Minimum valid distance along the ray
    /// * `tmax` - Maximum valid distance along the ray
    ///
    /// # Returns
    /// `true` if the ray intersects the AABB within [tmin, tmax]
    #[must_use]
    pub fn intersects(&self, r: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        // Test intersection with each pair of planes
        for i in 0..3 {
            // Calculate t values where ray intersects the slab planes
            // t = (plane_position - ray_origin) / ray_direction
            let t1 = (self.min[i] - r.origin[i]) / r.dir[i];
            let t2 = (self.max[i] - r.origin[i]) / r.dir[i];

            // Update the intersection interval
            // We need the ray to be inside all three slabs simultaneously
            tmin = t1.min(t2).max(tmin); // Latest entry point
            tmax = t1.max(t2).min(tmax); // Earliest exit point

            // If ray exits before it enters, there's no intersection
            if tmax < tmin {
                return false;
            }
        }
        true
    }

    /// Creates a new AABB that contains both input boxes.
    ///
    /// Used when building BVH nodes to create parent bounding boxes
    /// that encompass all child geometry.
    ///
    /// # Arguments
    /// * `box0` - First bounding box
    /// * `box1` - Second bounding box
    ///
    /// # Returns
    /// A new AABB with minimum corner being the component-wise minimum
    /// of both boxes, and maximum being the component-wise maximum.
    #[must_use]
    pub fn get_surrounding_aabb(box0: &Self, box1: &Self) -> Self {
        // Find the minimum corner (smallest values in each dimension)
        let small = Point3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        // Find the maximum corner (largest values in each dimension)
        let big = Point3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        Self::new(small, big)
    }
}
