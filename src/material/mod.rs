//! Material definitions for ray tracing.
//!
//! This module provides various material types that define how surfaces
//! interact with light, including diffuse, specular, and emissive properties.
//!
//! ## Available Materials
//! - `Matte`: Purely diffuse (Lambertian) surfaces
//! - `Phong`: Classic Phong shading with ambient, diffuse, and specular components
//! - `Reflective`: Materials with perfect or glossy reflections
//! - `Emissive`: Light-emitting surfaces

mod emissive;
mod matte;
mod phong;
mod reflective;

pub use emissive::*;
pub use matte::*;
pub use phong::*;
pub use reflective::*;

use crate::{color::Color, light::Light, model::Vec3, ray::Hit};

/// Trait for all materials that define surface properties.
///
/// Materials determine how surfaces interact with light through
/// various components:
/// - Ambient: Base color under ambient lighting
/// - Diffuse: Matte reflection (Lambertian)
/// - Specular: Mirror-like reflection (highlights)
/// - Reflective: Perfect or glossy reflections of the environment
/// - Emissive: Light emission from the surface
pub trait Material: Send + Sync {
    /// Computes the final color for this material at the hit point.
    ///
    /// This is the main entry point that combines all material properties.
    fn shade(&self, hit: &Hit) -> Color {
        shade(self, hit)
    }

    /// Returns whether this material emits light.
    fn emissive(&self) -> bool {
        false
    }

    /// Returns the ambient color component.
    ///
    /// This is the base color visible under ambient lighting.
    fn ambient(&self) -> Color {
        Color::zeros()
    }

    /// Computes the diffuse (Lambertian) reflection.
    ///
    /// # Arguments
    /// * `hit` - The hit point information
    /// * `wi` - Incoming light direction (from surface to light)
    fn diffuse(&self, _hit: &Hit, _wi: &Vec3) -> Color {
        Color::zeros()
    }

    /// Computes the specular (mirror-like) reflection.
    ///
    /// # Arguments
    /// * `hit` - The hit point information
    /// * `wi` - Incoming light direction (from surface to light)
    fn specular(&self, _hit: &Hit, _wi: &Vec3) -> Color {
        Color::zeros()
    }

    /// Computes reflected color from the environment.
    ///
    /// This handles perfect or glossy reflections by casting new rays.
    fn reflective(&self, _hit: &Hit) -> Color {
        Color::zeros()
    }
}

/// Generic shading function that computes the final color for any material.
///
/// This implements the core shading equation:
/// `color = ambient + Σ(lights) [N·L * (diffuse + specular) * radiance * shadow] + reflective`
///
/// Where:
/// - N·L is the cosine of the angle between surface normal and light direction
/// - radiance is the incoming light intensity
/// - shadow is the shadow attenuation factor
///
/// # Arguments
/// * `m` - The material to shade
/// * `hit` - Information about the ray-surface intersection
pub fn shade<M: Material + ?Sized>(m: &M, hit: &Hit) -> Color {
    // Optimized shading with better cache locality and vectorization

    // Start with ambient contribution
    let mut total_color =
        m.ambient().component_mul(&hit.renderer.scene.ambient_light().radiance(hit));

    // Pre-compute reflective contribution once (often zero, so check first)
    let reflective = m.reflective(hit);
    let has_reflection = reflective != Color::zeros();

    // Accumulate light contributions with better loop efficiency
    for light in hit.renderer.scene.lights() {
        // Get direction from surface to light
        let wi = light.get_direction(hit);

        // Calculate angle between surface normal and light direction
        let ndotwi = hit.normal.dot(&wi);

        // Early exit if light is behind the surface
        if ndotwi <= 0.0 {
            continue;
        }

        // Get light intensity at hit point
        let radiance = light.radiance(hit);

        // Early exit if no radiance
        if radiance == Vec3::zeros() {
            continue;
        }

        // Calculate shadow attenuation
        let shadow = light.shadow_amount(hit);

        // Skip if completely in shadow
        if shadow == 0.0 {
            continue;
        }

        // Compute material response to light
        let diffuse = m.diffuse(hit, &wi);
        let specular = m.specular(hit, &wi);

        // Apply lighting equation with optimized operations
        // Combine operations to improve cache usage and vectorization
        let material_response = diffuse + specular;
        let light_contribution = radiance * shadow * ndotwi;
        total_color += material_response.component_mul(&light_contribution);

        // Add reflection contribution per light
        if has_reflection {
            total_color += reflective;
        }
    }

    total_color
}
