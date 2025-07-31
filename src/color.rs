//! Color representation and operations for the raytracer.

use nalgebra::Vector3;

/// RGB color represented as a 3D vector with f64 components
pub type Color = Vector3<f64>;

/// Maximum RGB component value for tone mapping
const MAX_RGB_VALUE: f64 = 255.0;

/// Converts a floating-point color to RGB byte values
#[must_use]
pub fn to_rgb(color: &Color) -> Vec<u8> {
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    tone_mapping(color).iter().map(|c| ((c * MAX_RGB_VALUE).round() as u8).clamp(0, 255)).collect()
}

/// Applies tone mapping to prevent color clipping and maintain relative intensities
fn tone_mapping(color: &Color) -> Color {
    let max = color.x.max(color.y).max(color.z).max(1.0);
    color / max
}
