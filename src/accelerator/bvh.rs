//! Bounding Volume Hierarchy (BVH) acceleration structure.
//!
//! BVH is a tree structure used to accelerate ray-object intersection tests
//! by organizing geometric objects into a hierarchy of bounding volumes.
//! This significantly reduces the number of intersection tests needed.

use std::sync::Arc;

use nalgebra::Point3;
use rand::{Rng, rng};

use crate::{
    aabb::Aabb,
    geometric_object::Geometry,
    ray::{HitRecord, Ray},
};

/// A node in the Bounding Volume Hierarchy tree.
///
/// Each node contains either:
/// - Two child nodes (internal node)
/// - A single geometry object (leaf node)
///
/// The node stores an axis-aligned bounding box that encompasses
/// all geometry contained within its subtree.
pub struct Bvh {
    /// Left child node or geometry
    pub left: Arc<dyn Geometry>,
    /// Right child node or geometry
    pub right: Arc<dyn Geometry>,
    /// Bounding box encompassing both children
    pub aabb: Aabb,
}

impl Bvh {
    /// Constructs a BVH tree from a list of geometric objects.
    ///
    /// This uses a top-down construction approach:
    /// 1. Randomly select a splitting axis (X, Y, or Z)
    /// 2. Sort objects along that axis
    /// 3. Split objects into two groups at the median
    /// 4. Recursively build left and right subtrees
    ///
    /// # Arguments
    /// * `objects` - Vector of geometric objects to organize
    ///
    /// # Returns
    /// The root node of the constructed BVH tree
    ///
    /// # Panics
    /// * if `partial_cmp` fails (shouldn't happen with valid geometry)
    #[must_use]
    pub fn construct(mut objects: Vec<Arc<dyn Geometry>>) -> Arc<dyn Geometry> {
        match objects.len() {
            // Should never happen - handled by caller
            0 => unreachable!(),
            // Single object becomes a leaf node
            1 => objects.remove(0),
            // Multiple objects: create internal node
            _ => {
                // Randomly choose axis for spatial median split (SAH could be better)
                let axis = rng().random_range(0..3);

                // Sort objects along chosen axis using bounding box minimum
                objects.sort_by(|a, b| {
                    a.get_bounding_box().min[axis]
                        .partial_cmp(&b.get_bounding_box().min[axis])
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                // Split objects at median
                let mut a = objects;
                let b = a.split_off(a.len() / 2);

                // Recursively build subtrees
                let left = Self::construct(a);
                let right = Self::construct(b);

                // Create bounding box that encompasses both children
                let aabb =
                    Aabb::get_surrounding_aabb(&left.get_bounding_box(), &right.get_bounding_box());

                Arc::new(Self { left, right, aabb })
            }
        }
    }
}

impl Geometry for Bvh {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        // Early exit if ray doesn't hit the bounding box
        if !self.aabb.intersects(ray, t_min, t_max) {
            return None;
        }

        // Check left child first
        self.left.intersects(ray, t_min, t_max).map_or_else(
            // No left hit: just check right child
            || self.right.intersects(ray, t_min, t_max),
            // Left hit found: check if right child has closer hit
            |r1| self.right.intersects(ray, t_min, r1.dist).or(Some(r1)),
        )
    }

    fn get_min_point(&self) -> Point3<f64> {
        self.aabb.min
    }

    fn get_max_point(&self) -> Point3<f64> {
        self.aabb.max
    }
}
