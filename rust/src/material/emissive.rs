use crate::color::Color;
use std::ops::Mul;

pub struct Emissive {
    ls: f64, // radiance scaling factor
    ce: Color,
}

impl Emissive {
    pub fn new(ls: f64, ce: Color) -> Emissive {
        return Emissive { ls, ce };
    }

    pub fn radiance(&self) -> Color {
        return self.ce.mul(self.ls);
    }
}
