use super::Brdf;
use crate::{color::Color, model::Vec3, ray::Hit};

/// Shows specular highlights (shiny white dot) on surfaces
pub struct GlossySpecular {
    /// specular reflection coefficient [0, 1]
    pub ks: f64,

    /// shininess
    pub exp: f64,

    /// specular color
    pub cs: Color,
}

impl GlossySpecular {
    #[must_use]
    pub const fn new(ks: f64, exp: f64, cs: Color) -> Self {
        Self { ks, exp, cs }
    }
}

impl Brdf for GlossySpecular {
    /// Chapter 15
    fn f(&self, hit: &Hit, wi: &Vec3) -> Color {
        let wo = -hit.ray.dir;
        let ndotwi = hit.normal.dot(wi).max(0.0);
        let r = hit.normal * (2.0 * ndotwi) - wi;
        let rdotwo = r.dot(&wo);
        if rdotwo <= 0.0 {
            return Color::zeros();
        }
        let s = self.ks * rdotwo.powf(self.exp);
        Color::repeat(s)
    }

    /// Chapter 25
    fn sample_f(&self, hit: &Hit, wi: &mut Vec3, pdf: &mut f64) -> Color {
        let wo = -hit.ray.dir;
        let ndotwo = hit.normal.dot(&wo);
        // direction of mirror reflection
        let r = hit.normal * (2.0 * ndotwo) - wo;
        let w = r;
        let u = Vec3::new(0.00424, 1.0, 0.00764).cross(&w).normalize();
        let v = u.cross(&w);
        let sp = hit.renderer.sampler.hemisphere().take(1).collect::<Vec<_>>().remove(0);
        // reflected ray direction
        *wi = sp.x * u + sp.y * v + sp.z * w;
        // reflected ray is below surface
        if wi < &mut Vec3::zeros() {
            *wi = -sp.x * u - sp.y * v + sp.z * w;
        }
        let phong_lobe = r.dot(wi).powf(self.exp);
        *pdf = phong_lobe * hit.normal.dot(wi);
        self.cs * self.ks * phong_lobe
    }
}
