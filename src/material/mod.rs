mod emissive;
mod matte;
mod phong;
mod reflective;

pub use emissive::*;
pub use matte::*;
pub use phong::*;
pub use reflective::*;

use crate::color::Color;
use crate::light::Light;
use crate::model::Vec3;
use crate::ray::Hit;

pub trait Material: Send + Sync {
    fn shade(&self, hit: &Hit) -> Color {
        shade(self, hit)
    }

    fn emissive(&self) -> bool {
        false
    }

    fn ambient(&self) -> Color {
        Color::zeros()
    }

    fn diffuse(&self, _hit: &Hit, _wi: &Vec3) -> Color {
        Color::zeros()
    }

    fn specular(&self, _hit: &Hit, _wi: &Vec3) -> Color {
        Color::zeros()
    }

    fn reflective(&self, _hit: &Hit) -> Color {
        Color::zeros()
    }
}

pub fn shade<M: Material + ?Sized>(m: &M, hit: &Hit) -> Color {
    let light_color = hit
        .renderer
        .scene
        .lights
        .iter()
        .map(|light| {
            // wi: incoming direction
            let wi = light.get_direction(hit);

            // ndotwi: angle between light and normal
            let ndotwi = hit.normal.dot(&wi);

            // not hit by light
            if ndotwi <= 0.0 {
                return Color::zeros();
            }

            let radiance = light.radiance(hit);
            if radiance <= Vec3::zeros() {
                return Color::zeros();
            }

            let shadow = light.shadow_amount(hit);
            let diffuse = m.diffuse(hit, &wi);
            let specular = m.specular(hit, &wi);
            let reflective = m.reflective(hit);
            let color = ndotwi * (diffuse + specular).component_mul(&(radiance * shadow));

            color + reflective
        })
        .sum::<Color>();

    let ambient_color = m
        .ambient()
        .component_mul(&hit.renderer.scene.ambient_light.radiance(hit));

    ambient_color + light_color
}
