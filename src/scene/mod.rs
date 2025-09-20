//! Scene definitions and management for the ray tracer.
//!
//! This module provides scene structures that combine:
//! - Geometric objects
//! - Lights
//! - Camera configuration
//! - Acceleration structures
//!
//! ## Available Scenes
//! - `CornellBox`: Classic Cornell Box scene for testing global illumination

mod cornell_box;
mod factory;

pub use cornell_box::*;
pub use factory::*;

use crate::{
    camera::Camera,
    light::{Ambient, Light},
    ray::{HitRecord, Ray},
};
use std::sync::Arc;

/// Trait for all scenes that can be rendered.
///
/// A scene contains all the elements needed for rendering:
/// - Geometric objects (organized in acceleration structures)
/// - Light sources
/// - Camera configuration
/// - View settings
pub trait Scene: Send + Sync {
    /// Returns the width of the rendered image in pixels.
    fn view_width(&self) -> u32;

    /// Returns the height of the rendered image in pixels.
    fn view_height(&self) -> u32;

    /// Returns the camera used to generate rays.
    fn camera(&self) -> &dyn Camera;

    /// Returns the ambient light for the scene.
    fn ambient_light(&self) -> &Arc<Ambient>;

    /// Returns all light sources in the scene.
    fn lights(&self) -> &[Arc<dyn Light>];

    /// Tests for ray-object intersection in the scene.
    ///
    /// # Arguments
    /// * `ray` - The ray to test
    /// * `t_min` - Minimum valid distance along the ray
    /// * `t_max` - Maximum valid distance along the ray
    ///
    /// # Returns
    /// The closest intersection, if any.
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>>;
}
