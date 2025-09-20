use nalgebra::Point2;
use rayon::prelude::*;

use crate::{
    args::Args,
    color::Color,
    config::render::{DEFAULT_MAX_DEPTH, PREVIEW_MAX_DEPTH, PREVIEW_SAMPLES},
    ray::{Hit, Ray},
    sampler::Sampler,
    scene::Scene,
};

/// The main rendering engine that traces rays through a scene
pub struct Renderer {
    /// The scene to render
    pub scene: Box<dyn Scene>,
    /// The sampler for antialiasing and Monte Carlo integration
    pub sampler: Sampler,
    /// Maximum recursion depth for ray bounces
    max_depth: u8,
}

impl Renderer {
    /// Creates a new renderer with the given scene and configuration
    #[must_use]
    pub fn new(scene: Box<dyn Scene>, args: &Args) -> Self {
        Self {
            scene,
            sampler: Sampler::new(if args.preview { PREVIEW_SAMPLES } else { args.samples }),
            max_depth: if args.preview { PREVIEW_MAX_DEPTH } else { DEFAULT_MAX_DEPTH },
        }
    }

    /// Returns the maximum ray tracing depth
    #[must_use]
    pub const fn max_depth(&self) -> u8 {
        self.max_depth
    }

    /// Renders the scene and returns a vector of colors for each pixel
    #[must_use]
    pub fn render(&self) -> Vec<Color> {
        let width = self.scene.view_width();
        let height = self.scene.view_height();
        let pixel_size = self.scene.camera().setting().pixel_size;

        let vec = (0..(width * height))
            .into_par_iter()
            .flat_map_iter(|n| {
                let i = pixel_size * (f64::from(n % width) - f64::from(width) / 2.0);
                let j = pixel_size * (f64::from(n / width) - f64::from(height) / 2.0);
                let origin = Point2::new(i, j);
                self.scene.camera().get_rays(origin, &self.sampler).into_iter()
            })
            .map(|ray| self.trace(&ray, 0))
            .collect::<Vec<_>>();

        vec.chunks(self.sampler.count().into())
            .map(|chunks| chunks.iter().sum::<Color>() / f64::from(self.sampler.count()))
            .collect()
    }

    /// Traces a ray through the scene and returns the resulting color
    ///
    /// # Arguments
    /// * `ray` - The ray to trace
    /// * `depth` - Current recursion depth
    ///
    /// # Returns
    /// The color contribution from this ray
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
