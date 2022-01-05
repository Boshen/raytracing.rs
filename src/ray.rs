use nalgebra::Point3;

use crate::counter;
use crate::material::Material;
use crate::model::Vec3;
use crate::renderer::Renderer;

pub struct Ray {
    pub origin: Point3<f64>,
    pub dir: Vec3,
}

impl Ray {
    #[must_use]
    pub fn new(origin: Point3<f64>, dir: Vec3) -> Self {
        counter::inc_ray_count();
        Self { origin, dir }
    }

    #[must_use]
    pub fn get_point(&self, distance: f64) -> Point3<f64> {
        self.origin + self.dir * distance
    }
}

pub struct HitRecord<'a> {
    pub dist: f64,
    pub hit_point: Point3<f64>,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub struct Hit<'a> {
    pub ray: &'a Ray,
    pub hit_point: Point3<f64>,
    pub normal: Vec3,
    pub renderer: &'a Renderer,
    pub depth: u8,
    pub material: &'a dyn Material,
}
