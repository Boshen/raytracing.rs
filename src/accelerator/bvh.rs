//! Bounding Volume Hierarchy (BVH) acceleration structure.
//!
//! BVH is a tree structure used to accelerate ray-object intersection tests
//! by organizing geometric objects into a hierarchy of bounding volumes.
//! This significantly reduces the number of intersection tests needed.
//!
//! This implementation uses the Surface Area Heuristic (SAH) for optimal
//! tree construction, which minimizes the expected cost of ray traversal.

use std::sync::Arc;

use nalgebra::Point3;

use crate::{
    aabb::Aabb,
    config::bvh::{INTERSECTION_COST, MAX_DEPTH, MIN_PRIMITIVES_FOR_SPLIT, TRAVERSAL_COST},
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
    /// This uses the Surface Area Heuristic (SAH) for optimal tree construction:
    /// 1. Evaluate splitting along all three axes
    /// 2. Use binning to find the best split position
    /// 3. Choose the split that minimizes expected traversal cost
    /// 4. Recursively build left and right subtrees
    ///
    /// # Arguments
    /// * `objects` - Vector of geometric objects to organize
    ///
    /// # Returns
    /// The root node of the constructed BVH tree
    #[must_use]
    pub fn construct(objects: Vec<Arc<dyn Geometry>>) -> Arc<dyn Geometry> {
        Self::construct_recursive(objects, 0)
    }

    /// Recursively constructs the BVH tree using SAH.
    fn construct_recursive(mut objects: Vec<Arc<dyn Geometry>>, depth: usize) -> Arc<dyn Geometry> {
        match objects.len() {
            0 => unreachable!("BVH construction called with empty object list"),
            1 => objects.remove(0),
            2 => {
                // For just 2 objects, create a simple node
                let left = objects.remove(0);
                let right = objects.remove(0);
                let aabb =
                    Aabb::get_surrounding_aabb(&left.get_bounding_box(), &right.get_bounding_box());
                Arc::new(Self { left, right, aabb })
            }
            n if n < MIN_PRIMITIVES_FOR_SPLIT || depth >= MAX_DEPTH => {
                // Too few primitives or max depth - use median split
                Self::median_split(objects, depth)
            }
            _ => {
                // Use SAH for larger object counts
                let (axis, split_index) = Self::find_best_split_simplified(&objects);

                // Partition objects
                objects.sort_by(|a, b| {
                    let a_center = {
                        let bounds = a.get_bounding_box();
                        (bounds.min[axis] + bounds.max[axis]) * 0.5
                    };
                    let b_center = {
                        let bounds = b.get_bounding_box();
                        (bounds.min[axis] + bounds.max[axis]) * 0.5
                    };
                    a_center.partial_cmp(&b_center).unwrap_or(std::cmp::Ordering::Equal)
                });

                let right_objects = objects.split_off(split_index);

                // Recursively build subtrees
                let left = Self::construct_recursive(objects, depth + 1);
                let right = Self::construct_recursive(right_objects, depth + 1);

                let aabb =
                    Aabb::get_surrounding_aabb(&left.get_bounding_box(), &right.get_bounding_box());

                Arc::new(Self { left, right, aabb })
            }
        }
    }

    /// Simplified SAH split finding without caching
    #[expect(clippy::cast_precision_loss, reason = "Acceptable for SAH cost calculation")]
    fn find_best_split_simplified(objects: &[Arc<dyn Geometry>]) -> (usize, usize) {
        let mut best_axis = 0;
        let mut best_index = objects.len() / 2;
        let mut best_cost = f64::INFINITY;

        // Compute total bounds
        let mut total_bounds = objects[0].get_bounding_box();
        for obj in &objects[1..] {
            total_bounds = Aabb::get_surrounding_aabb(&total_bounds, &obj.get_bounding_box());
        }

        // Early exit for degenerate cases
        let extent = total_bounds.max - total_bounds.min;
        if extent.x < f64::EPSILON && extent.y < f64::EPSILON && extent.z < f64::EPSILON {
            return (0, objects.len() / 2);
        }

        let total_area = total_bounds.surface_area();
        if total_area < f64::EPSILON {
            return (0, objects.len() / 2);
        }

        let inv_total_area = 1.0 / total_area;

        // Try each axis
        for axis in 0..3 {
            // Skip degenerate axes
            if extent[axis] < f64::EPSILON {
                continue;
            }

            // Create sorted indices for this axis
            let mut indices: Vec<usize> = (0..objects.len()).collect();
            indices.sort_by(|&a, &b| {
                let a_center = {
                    let bounds = objects[a].get_bounding_box();
                    (bounds.min[axis] + bounds.max[axis]) * 0.5
                };
                let b_center = {
                    let bounds = objects[b].get_bounding_box();
                    (bounds.min[axis] + bounds.max[axis]) * 0.5
                };
                a_center.partial_cmp(&b_center).unwrap_or(std::cmp::Ordering::Equal)
            });

            // Test a few split positions (not all, for performance)
            let test_splits = [objects.len() / 4, objects.len() / 2, 3 * objects.len() / 4];

            for &split_index in &test_splits {
                if split_index == 0 || split_index >= objects.len() {
                    continue;
                }

                // Compute left bounds
                let mut left_bounds = objects[indices[0]].get_bounding_box();
                for i in 1..split_index {
                    left_bounds = Aabb::get_surrounding_aabb(
                        &left_bounds,
                        &objects[indices[i]].get_bounding_box(),
                    );
                }

                // Compute right bounds
                let mut right_bounds = objects[indices[split_index]].get_bounding_box();
                for i in split_index + 1..indices.len() {
                    right_bounds = Aabb::get_surrounding_aabb(
                        &right_bounds,
                        &objects[indices[i]].get_bounding_box(),
                    );
                }

                // Calculate SAH cost
                let left_area = left_bounds.surface_area();
                let right_area = right_bounds.surface_area();
                let cost = TRAVERSAL_COST
                    + (left_area * inv_total_area * split_index as f64
                        + right_area * inv_total_area * (objects.len() - split_index) as f64)
                        * INTERSECTION_COST;

                if cost < best_cost {
                    best_cost = cost;
                    best_axis = axis;
                    best_index = split_index;
                }
            }
        }

        (best_axis, best_index)
    }

    /// Simple median split for small object counts
    fn median_split(mut objects: Vec<Arc<dyn Geometry>>, depth: usize) -> Arc<dyn Geometry> {
        // Find longest axis
        let mut bounds = objects[0].get_bounding_box();
        for obj in &objects[1..] {
            bounds = Aabb::get_surrounding_aabb(&bounds, &obj.get_bounding_box());
        }

        let extent = bounds.max - bounds.min;
        let axis = if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        };

        // Sort and split at median
        objects.sort_by(|a, b| {
            let a_center = {
                let bounds = a.get_bounding_box();
                (bounds.min[axis] + bounds.max[axis]) * 0.5
            };
            let b_center = {
                let bounds = b.get_bounding_box();
                (bounds.min[axis] + bounds.max[axis]) * 0.5
            };
            a_center.partial_cmp(&b_center).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mid = objects.len() / 2;
        let right_objects = objects.split_off(mid);

        let left = Self::construct_recursive(objects, depth + 1);
        let right = Self::construct_recursive(right_objects, depth + 1);

        let aabb = Aabb::get_surrounding_aabb(&left.get_bounding_box(), &right.get_bounding_box());

        Arc::new(Self { left, right, aabb })
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
