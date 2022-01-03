mod glossy_specular;
mod lambertian;
mod perfect_specular;

pub use glossy_specular::*;
pub use lambertian::*;
pub use perfect_specular::*;

use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

/// Bidrectional Reflectance Distribution Function
/// describes how light is reflected at surfaces for materials
pub trait Brdf {
    /// Reciprocity
    fn f(&self, _hit: &Hit, _wi: &Vec3) -> Color {
        Color::zeros()
    }

    /// Bihemispherical reflectance
    fn rho(&self) -> Color {
        Color::zeros()
    }

    /// The direction of reflected rays for simulating reflective materials
    fn sample_f(&self, _hit: &Hit, _wi: &Vec3) -> Vec3 {
        Color::zeros()
    }
}
