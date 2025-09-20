//! # Raytracing in Rust 🦀
//!
//! A high-performance ray tracer implementing concepts from "Ray Tracing from the Ground Up".
//!
//! This library provides a complete ray tracing system with:
//! - Multiple geometric primitives (spheres, triangles)
//! - Various materials (matte, phong, reflective, emissive)
//! - Different lighting models
//! - Acceleration structures (BVH)
//! - Multiple camera types (simple, thin lens)
//! - Parallel rendering using Rayon
//!
//! ## Example
//!
//! ```rust
//! use raytracing::{args::Args, renderer::Renderer, scene::CornellBox};
//!
//! let args = Args {
//!     width: 100,
//!     height: 100,
//!     preview: true,
//!     camera: raytracing::args::ArgCamera::ThinLens,
//!     samples: 4,
//! };
//! let scene = CornellBox::new(args.width, args.height, &args).unwrap();
//! let renderer = Renderer::new(Box::new(scene), &args);
//! let pixels = renderer.render();
//! ```

/// Axis-aligned bounding box functionality for acceleration structures
pub mod aabb;
/// Spatial acceleration structures like BVH for fast ray-object intersection
pub mod accelerator;
/// Command line argument parsing and configuration
pub mod args;
/// Asset loading and management utilities
pub mod asset;
/// Bidirectional Reflectance Distribution Functions for realistic material rendering
pub mod brdf;
/// Camera models and ray generation
pub mod camera;
/// Color representation and operations
pub mod color;
/// Configuration constants and settings
pub mod config;
/// Error types and handling for the raytracing library
pub mod error;
/// Geometric objects that can be rendered (spheres, triangles, etc.)
pub mod geometric_object;
/// Light sources and illumination models
pub mod light;
/// Material properties and shading models
pub mod material;
/// Mathematical models and vector operations
pub mod model;
/// Ray representation and hit testing
pub mod ray;
/// Main rendering engine and ray tracing logic
pub mod renderer;
/// Sampling strategies for antialiasing and Monte Carlo integration
pub mod sampler;
/// Scene setup and management
pub mod scene;

#[cfg(test)]
mod tests {
    use crate::{
        args::{ArgCamera, Args},
        renderer::Renderer,
        scene::CornellBox,
    };

    #[test]
    fn render_basic() {
        let args =
            Args { width: 10, height: 10, preview: false, camera: ArgCamera::ThinLens, samples: 4 };
        let scene = CornellBox::new(args.width, args.height, &args).unwrap();
        let renderer = Renderer::new(Box::new(scene), &args);
        let pixels = renderer.render();
        assert_eq!(pixels.len(), 100);
    }

    #[test]
    fn render_preview_mode() {
        let args =
            Args { width: 5, height: 5, preview: true, camera: ArgCamera::Simple, samples: 1 };
        let scene = CornellBox::new(args.width, args.height, &args).unwrap();
        let renderer = Renderer::new(Box::new(scene), &args);
        let pixels = renderer.render();
        assert_eq!(pixels.len(), 25);
        // Preview mode should use 1 sample and depth 1
        assert_eq!(renderer.sampler.count(), 1);
        assert_eq!(renderer.max_depth(), 1);
    }

    #[test]
    fn render_different_cameras() {
        for camera in [ArgCamera::Simple, ArgCamera::ThinLens] {
            let args = Args { width: 3, height: 3, preview: true, camera, samples: 1 };
            let scene = CornellBox::new(args.width, args.height, &args).unwrap();
            let renderer = Renderer::new(Box::new(scene), &args);
            let pixels = renderer.render();
            assert_eq!(pixels.len(), 9);
        }
    }

    #[test]
    fn render_different_sample_counts() {
        for samples in [1, 2, 4, 8] {
            let args =
                Args { width: 2, height: 2, preview: false, camera: ArgCamera::Simple, samples };
            let scene = CornellBox::new(args.width, args.height, &args).unwrap();
            let renderer = Renderer::new(Box::new(scene), &args);
            let pixels = renderer.render();
            assert_eq!(pixels.len(), 4);
            assert_eq!(renderer.sampler.count(), samples);
        }
    }
}
