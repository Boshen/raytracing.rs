use super::Brdf;
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct GlossySpecular {
    /// specular reflection coefficient [0, 1]
    pub ks: f64,

    /// shininess
    pub exp: f64,
}

impl GlossySpecular {
    #[must_use]
    pub const fn new(ks: f64, exp: f64) -> Self {
        Self { ks, exp }
    }
}

impl Brdf for GlossySpecular {
    fn f(&self, hit: &Hit, wo: &Vec3, wi: &Vec3) -> Color {
        let ndotwi = hit.normal.dot(wi).max(0.0);
        let r = hit.normal * (2.0 * ndotwi) - wi;
        let rdotwo = r.dot(wo);
        if rdotwo <= 0.0 {
            return Color::zeros();
        }
        let s = self.ks * rdotwo.powf(self.exp);
        Color::repeat(s)
    }
}
