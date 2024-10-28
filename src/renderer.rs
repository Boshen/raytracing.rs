use nalgebra::Point2;
use rayon::prelude::*;

use crate::{
    args::Args,
    color::Color,
    ray::{Hit, Ray},
    sampler::Sampler,
    scene::CornellBox,
};

pub struct Renderer {
    pub scene: CornellBox,
    pub sampler: Sampler,
    max_depth: u8,
}

impl Renderer {
    #[must_use]
    pub fn new(scene: CornellBox, args: &Args) -> Self {
        Self {
            scene,
            sampler: Sampler::new(if args.preview { 1 } else { args.samples }),
            max_depth: if args.preview { 1 } else { 5 },
        }
    }

    #[must_use]
    pub fn render(&self) -> Vec<Color> {
        let width = self.scene.view_width;
        let height = self.scene.view_height;
        let pixel_size = self.scene.camera.setting().pixel_size;

        let vec = (0..(width * height))
            .into_par_iter()
            .flat_map_iter(|n| {
                let i = pixel_size * (f64::from(n % width) - f64::from(width) / 2.0);
                let j = pixel_size * (f64::from(n / width) - f64::from(height) / 2.0);
                let origin = Point2::new(i, j);
                self.scene.camera.get_rays(origin, &self.sampler).into_iter()
            })
            .map(|ray| self.trace(&ray, 0))
            .collect::<Vec<_>>();

        vec.chunks(self.sampler.count().into())
            .map(|chunks| chunks.iter().sum::<Color>() / f64::from(self.sampler.count()))
            .collect()
    }

    #[must_use]
    pub fn trace(&self, ray: &Ray, depth: u8) -> Color {
        if depth > self.max_depth {
            return Color::zeros();
        }
        self.scene.intersects(ray, 0.0, f64::INFINITY).map_or_else(Color::zeros, |record| {
            let wo = -ray.dir;
            // revert normal if we hit the inside surface
            let adjusted_normal = record.normal * record.normal.dot(&wo).signum();
            let rayhit = Hit {
                ray,
                hit_point: record.hit_point,
                material: record.material,
                normal: adjusted_normal,
                renderer: self,
                depth,
            };
            record.material.shade(&rayhit)
        })
    }
}
