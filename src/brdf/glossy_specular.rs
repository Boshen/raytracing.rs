use super::Brdf;
use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub struct GlossySpecular {
    pub ks: f64,  // specular reflection coefficient [0, 1]
    pub exp: f64, // shininess
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
        Color::new(s, s, s)
    }

    fn rho(&self) -> Color {
        Color::zeros() // is black for GlossySpecular
    }

    fn sample_f(&self, _hit: &Hit, _wo: &Vec3, _wi: &Vec3) -> Color {
        Color::zeros()
    }
}
