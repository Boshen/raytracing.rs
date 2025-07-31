//! Error types for the raytracing library.

use std::fmt;

/// Errors that can occur during ray tracing operations.
#[derive(Debug)]
pub enum RayTracingError {
    /// Error occurred during image processing or saving
    ImageError(image::ImageError),
    /// Error occurred during scene setup or rendering
    RenderError(String),
    /// Invalid configuration or arguments
    ConfigError(String),
}

impl fmt::Display for RayTracingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageError(err) => write!(f, "Image processing error: {err}"),
            Self::RenderError(msg) => write!(f, "Render error: {msg}"),
            Self::ConfigError(msg) => write!(f, "Configuration error: {msg}"),
        }
    }
}

impl std::error::Error for RayTracingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ImageError(err) => Some(err),
            Self::RenderError(_) | Self::ConfigError(_) => None,
        }
    }
}

impl From<image::ImageError> for RayTracingError {
    fn from(err: image::ImageError) -> Self {
        Self::ImageError(err)
    }
}

/// Result type alias for raytracing operations.
pub type Result<T> = std::result::Result<T, RayTracingError>;
