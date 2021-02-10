use crate::color::Color;

pub struct Emissive {
    pub ls: f64, // radiance scaling factor
    pub ce: Color,
}

impl Emissive {
    pub fn new(ls: f64, ce: Color) -> Emissive {
        Emissive { ls, ce }
    }

    pub fn radiance(&self) -> Color {
        self.ce * self.ls
    }
}
