//! Acceleration data structures for efficient ray-object intersection.
//!
//! This module provides spatial data structures like Bounding Volume Hierarchies (BVH)
//! that significantly speed up ray tracing by reducing the number of intersection tests.

mod bvh;

pub use bvh::*;
