use nalgebra::{clamp, Vector3};

pub type Color = Vector3<f64>;

pub fn to_rgb(color: &Color) -> Vec<u8> {
    tone_mapping(color)
        .iter()
        .map(|c| (c * 255.0).round() as u8)
        .map(|c| clamp(c, 0, 255))
        .collect()
}

fn tone_mapping(color: &Color) -> Color {
    let max = color.x.max(color.y).max(color.z).max(1.0);
    color / max
}
