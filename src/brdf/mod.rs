mod glossy_specular;
mod lambertian;
mod perfect_specular;

pub use glossy_specular::*;
pub use lambertian::*;
pub use perfect_specular::*;

use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub trait Brdf {
    // reciprocity
    fn f(&self, hit: &Hit, wo: &Vec3, wi: &Vec3) -> Color;
    // bihemispherical reflectance
    fn rho(&self) -> Color;
    fn sample_f(&self, hit: &Hit, wo: &Vec3, wi: &Vec3) -> Color;
}
