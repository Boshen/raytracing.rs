use crate::color::Color;
use crate::model::Vec3;
use crate::ray::Hit;

pub mod emissive;
pub mod matte;
pub mod phong;
pub mod reflective;

pub use emissive::*;
pub use matte::*;
pub use phong::*;
pub use reflective::*;

pub trait Material: Send + Sync {
    fn shade(&self, hit: &Hit) -> Color;
    fn emissive(&self) -> bool;
    fn ambient(&self, hit: &Hit) -> Color;
    fn diffuse(&self, hit: &Hit, wo: &Vec3, wi: &Vec3) -> Color;
    fn specular(&self, hit: &Hit, wo: &Vec3, wi: &Vec3) -> Color;
    fn reflective(&self, hit: &Hit, wo: &Vec3) -> Color;
}

pub fn shade(m: &dyn Material, hit: &Hit) -> Color {
    let ambient = m.ambient(hit);
    let color = hit
        .world
        .lights
        .iter()
        .map(|light| {
            // wi: incoming direction
            // ndotwi: angle between light and normal
            let wi = light.get_direction(hit);
            let ndotwi = hit.normal.dot(&wi);
            // not hit by light
            if ndotwi <= 0.0 {
                return Color::zeros();
            }

            let radiance = light.radiance(hit);
            if radiance <= Vec3::zeros() {
                return Color::zeros();
            }

            // wo: reflected direction
            let shadow_amount = light.shadow_amount(hit);

            let wo = (hit.ray.dir * -1.0).normalize();
            (m.diffuse(hit, &wo, &wi) + m.specular(hit, &wo, &wi))
                .component_mul(&(radiance * shadow_amount))
                * ndotwi
                + m.reflective(hit, &wo)
        })
        .sum::<Color>();
    ambient + color
}
