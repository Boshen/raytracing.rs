//! Color representation and operations for the raytracer.

use nalgebra::Vector3;

/// RGB color represented as a 3D vector with f64 components
pub type Color = Vector3<f64>;

/// Maximum RGB component value for tone mapping
const MAX_RGB_VALUE: f64 = 255.0;

/// Converts a floating-point color to RGB byte values.
///
/// Applies tone mapping first to handle HDR colors, then converts
/// each component to 0-255 range for standard image formats.
///
/// # Arguments
/// * `color` - HDR color with components in [0, ∞)
///
/// # Returns
/// Vector of 3 bytes (R, G, B) in [0, 255] range
#[must_use]
pub fn to_rgb(color: &Color) -> Vec<u8> {
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    tone_mapping(color).iter().map(|c| ((c * MAX_RGB_VALUE).round() as u8).clamp(0, 255)).collect()
}

/// Applies simple tone mapping to handle HDR colors.
///
/// This uses a basic exposure tone mapping that scales the entire
/// color by the reciprocal of the maximum component. This preserves
/// color relationships while preventing any channel from exceeding 1.0.
///
/// # Arguments
/// * `color` - HDR color with potentially unbounded values
///
/// # Returns
/// LDR color with all components in [0, 1] range
fn tone_mapping(color: &Color) -> Color {
    // Find the maximum component, ensuring at least 1.0 to avoid division issues
    let max = color.x.max(color.y).max(color.z).max(1.0);
    // Scale down by the maximum to fit in [0, 1] range
    color / max
}
