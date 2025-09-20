//! Configuration constants and settings for the raytracer.
//!
//! This module centralizes all configuration values, magic numbers,
//! and default settings used throughout the raytracer, making them
//! easier to find and modify.

/// Rendering configuration constants
pub mod render {
    /// Default maximum ray tracing depth for non-preview renders
    pub const DEFAULT_MAX_DEPTH: u8 = 5;

    /// Maximum ray tracing depth for preview renders
    pub const PREVIEW_MAX_DEPTH: u8 = 1;

    /// Minimum sample count for preview renders
    pub const PREVIEW_SAMPLES: u8 = 1;

    /// Default output filename for rendered images
    pub const OUTPUT_FILENAME: &str = "output.png";
}

/// Scene configuration constants
pub mod scene {
    /// Cornell Box scale factor (traditionally 555 units)
    pub const CORNELL_BOX_SCALE: f64 = 555.0;

    /// Default Cornell Box asset path
    pub const CORNELL_BOX_ASSET_PATH: &str = "./assets/cornell_box.obj";

    /// Default ambient light strength
    pub const DEFAULT_AMBIENT_STRENGTH: f64 = 0.1;
}

/// Camera configuration constants
pub mod camera {
    /// Default focal length for cameras
    pub const DEFAULT_FOCAL_LENGTH: f64 = 520.0;

    /// Default thin lens radius (aperture)
    pub const DEFAULT_LENS_RADIUS: f64 = 0.001;

    /// Default focal distance for thin lens
    pub const DEFAULT_FOCAL_DISTANCE: f64 = 580.0;

    /// Default camera eye position
    pub const DEFAULT_EYE_POSITION: [f64; 3] = [0.0, 0.0, -3.0];

    /// Default camera look-at point
    pub const DEFAULT_LOOKAT_POSITION: [f64; 3] = [0.0, 0.0, 0.0];
}

/// Material configuration constants
pub mod material {
    /// Default diffuse reflectance for matte materials
    pub const DEFAULT_DIFFUSE_KD: f64 = 1.0;

    /// Default ambient reflectance for matte materials
    pub const DEFAULT_AMBIENT_KD: f64 = 0.5;
}

/// Geometry configuration constants
pub mod geometry {
    /// Default sphere positions and radii for Cornell Box scene
    pub mod spheres {
        /// Small reflective sphere radius
        pub const SMALL_SPHERE_RADIUS: f64 = 30.0;

        /// Small sphere position
        pub const SMALL_SPHERE_POSITION: [f64; 3] = [350.0, 30.0, 500.0];

        /// Large reflective sphere radius
        pub const LARGE_SPHERE_RADIUS: f64 = 60.0;

        /// Large sphere position
        pub const LARGE_SPHERE_POSITION: [f64; 3] = [200.0, 60.0, 400.0];
    }
}