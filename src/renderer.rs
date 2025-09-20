//! Main rendering engine using thread-local arena allocation for optimal performance.
//!
//! This renderer uses bumpalo-herd to provide thread-local bump allocators,
//! significantly reducing allocation overhead and improving cache locality.

use bumpalo_herd::Herd;
use nalgebra::Point2;
use rayon::prelude::*;

use crate::{
    args::Args,
    color::Color,
    ray::{Hit, Ray},
    sampler::Sampler,
    scene::CornellBox,
};

/// Default maximum ray tracing depth for non-preview renders
const DEFAULT_MAX_DEPTH: u8 = 5;
/// Maximum ray tracing depth for preview renders
const PREVIEW_MAX_DEPTH: u8 = 1;
/// Minimum sample count for preview renders
pub const PREVIEW_SAMPLES: u8 = 1;

/// The main rendering engine that traces rays through a scene.
///
/// Uses thread-local arena allocation for improved performance:
/// - Fast bump allocation (11 instructions vs hundreds)
/// - Better cache locality with contiguous memory
/// - Reduced fragmentation
/// - Batch deallocation when rendering completes
pub struct Renderer {
    /// The scene to render
    pub scene: CornellBox,
    /// The sampler for antialiasing and Monte Carlo integration
    pub sampler: Sampler,
    /// Maximum recursion depth for ray bounces
    pub max_depth: u8,
}

impl Renderer {
    /// Creates a new renderer with the given scene and configuration
    #[must_use]
    pub fn new(scene: CornellBox, args: &Args) -> Self {
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

    /// Renders the scene using thread-local arena allocation.
    ///
    /// This method uses bumpalo-herd to provide each thread with its own
    /// bump allocator, avoiding synchronization overhead while maintaining
    /// the benefits of arena allocation.
    #[must_use]
    pub fn render(&self) -> Vec<Color> {
        let width = self.scene.view_width;
        let height = self.scene.view_height;
        let pixel_size = self.scene.camera.setting().pixel_size;
        let total_pixels = (width * height) as usize;
        let samples_per_pixel = self.sampler.count() as usize;

        // Create a herd of thread-local arenas
        let herd = Herd::new();

        // Calculate pixels per thread for chunking
        let num_threads = rayon::current_num_threads();
        let pixels_per_thread = (total_pixels + num_threads - 1) / num_threads;

        // Process pixels in parallel using thread-local arenas
        let pixel_groups: Vec<Vec<Color>> = (0..total_pixels)
            .into_par_iter()
            .chunks(pixels_per_thread)
            .map(|pixel_indices| {
                // Get thread-local arena from the herd
                let member = herd.get();
                let arena = member.as_bump();

                // Allocate space for all samples in this chunk
                let mut chunk_colors = bumpalo::collections::Vec::with_capacity_in(
                    pixel_indices.len() * samples_per_pixel,
                    arena,
                );

                // Process each pixel in the chunk
                for n in pixel_indices {
                    let i = pixel_size * (f64::from(n as u32 % width) - f64::from(width) / 2.0);
                    let j = pixel_size * (f64::from(n as u32 / width) - f64::from(height) / 2.0);
                    let origin = Point2::new(i, j);

                    // Generate rays and trace them
                    for ray in self.scene.camera.get_rays(origin, &self.sampler) {
                        chunk_colors.push(self.trace(&ray, 0));
                    }
                }

                // Average samples for each pixel and collect results
                // We need to convert from arena allocation back to standard Vec
                chunk_colors
                    .chunks(samples_per_pixel)
                    .map(|samples| {
                        samples.iter().sum::<Color>() / f64::from(self.sampler.count())
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // Flatten all chunks into final image
        pixel_groups.into_iter().flatten().collect()
    }

    /// Traces a ray through the scene and returns the resulting color.
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