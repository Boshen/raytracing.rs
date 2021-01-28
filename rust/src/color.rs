use nalgebra::Vector3;
use std::ops::Div;

pub type Color = Vector3<f64>;

pub fn tone_mapping(color: &Color) -> Color {
    let max = color.x.max(color.y).max(color.z).max(1.0);
    return color.div(max);
}
