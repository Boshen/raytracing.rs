//! Command line argument parsing and configuration for the raytracer.

use bpaf::Bpaf;

/// Configuration options for the raytracer
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Args {
    /// Show a quick preview with minimal samples and depth.
    /// This significantly reduces render time for quick feedback.
    pub preview: bool,

    /// Width of the output image in pixels
    #[bpaf(fallback(500))]
    pub width: u32,

    /// Height of the output image in pixels
    #[bpaf(fallback(500))]
    pub height: u32,

    /// Camera type to use for rendering
    #[bpaf(external(arg_camera), fallback(ArgCamera::ThinLens))]
    pub camera: ArgCamera,

    /// Number of samples per pixel for antialiasing.
    /// Higher values produce better quality but take longer to render.
    #[bpaf(fallback(16))]
    pub samples: u8,
}

/// Available camera types for rendering
#[derive(Debug, Clone, Bpaf)]
pub enum ArgCamera {
    /// Simple pinhole camera model
    Simple,
    /// Thin lens camera model with depth of field effects
    ThinLens,
}

impl Args {
    /// Validates the configuration arguments
    ///
    /// # Errors
    /// Returns an error if any arguments are invalid
    pub fn validate(&self) -> Result<(), String> {
        if self.width == 0 || self.height == 0 {
            return Err("Width and height must be greater than 0".to_string());
        }

        if self.samples == 0 {
            return Err("Sample count must be greater than 0".to_string());
        }

        // Reasonable limits to prevent excessive memory usage
        if self.width > 8192 || self.height > 8192 {
            return Err("Width and height must be 8192 or less".to_string());
        }

        Ok(())
    }
}
