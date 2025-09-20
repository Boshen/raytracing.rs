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
    // Accumulate contributions from all lights
    let light_color = hit
        .renderer
        .scene
        .lights()
        .iter()
        .map(|light| {
            // Get direction from surface to light
            let wi = light.get_direction(hit);

            // Calculate angle between surface normal and light direction
            let ndotwi = hit.normal.dot(&wi);

            // Skip if light is behind the surface
            if ndotwi <= 0.0 {
                return Color::zeros();
            }

            // Get light intensity at hit point
            let radiance = light.radiance(hit);
            if radiance <= Vec3::zeros() {
                return Color::zeros();
            }

            // Calculate shadow attenuation
            let shadow = light.shadow_amount(hit);

            // Compute material response to light
            let diffuse = m.diffuse(hit, &wi);
            let specular = m.specular(hit, &wi);
            let reflective = m.reflective(hit);

            // Apply lighting equation
            let color = ndotwi * (diffuse + specular).component_mul(&(radiance * shadow));

            color + reflective
        })
        .sum::<Color>();

    // Add ambient lighting contribution
    let ambient_color =
        m.ambient().component_mul(&hit.renderer.scene.ambient_light().radiance(hit));

    ambient_color + light_color
}
