use nalgebra::Vector3;

pub type Color = Vector3<f64>;

#[must_use]
pub fn to_rgb(color: &Color) -> Vec<u8> {
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    tone_mapping(color)
        .iter()
        .map(|c| ((c * 255.0).round() as u8).clamp(0, 255))
        .collect()
}

fn tone_mapping(color: &Color) -> Color {
    let max = color.x.max(color.y).max(color.z).max(1.0);
    color / max
}
