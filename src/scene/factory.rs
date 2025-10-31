//! Scene factory for creating different scene types.
//!
//! This module provides a centralized way to create scenes based on
//! configuration parameters, making it easier to add new scene types
//! and manage scene creation logic.

use crate::{
    args::Args,
    error::Result,
    scene::{CornellBox, Scene},
};

/// Factory for creating different types of scenes.
pub struct SceneFactory;

impl SceneFactory {
    /// Creates a scene based on the provided arguments.
    ///
    /// Currently only supports Cornell Box, but this can be extended
    /// to support multiple scene types based on configuration.
    ///
    /// # Arguments
    /// * `args` - Configuration arguments for scene creation
    ///
    /// # Returns
    /// A boxed scene implementing the Scene trait
    ///
    /// # Errors
    /// Returns an error if the scene cannot be created or assets cannot be loaded
    pub fn create_scene(args: &Args) -> Result<Box<dyn Scene>> {
        // For now, only Cornell Box is supported
        // Future: Add scene selection logic here based on args
        Self::create_cornell_box(args)
    }

    /// Creates a Cornell Box scene.
    ///
    /// # Arguments
    /// * `args` - Configuration arguments for scene creation
    ///
    /// # Returns
    /// A boxed Cornell Box scene
    ///
    /// # Errors
    /// Returns an error if the Cornell Box scene cannot be created
    fn create_cornell_box(args: &Args) -> Result<Box<dyn Scene>> {
        // Note: Using height for both dimensions creates a square image
        // This is intentional for the Cornell Box scene
        let scene = CornellBox::new(args.height, args.height, args)?;
        Ok(Box::new(scene))
    }
}

/// Available scene types that can be created by the factory.
#[derive(Debug, Clone, Default)]
pub enum SceneType {
    /// Classic Cornell Box scene for testing global illumination
    #[default]
    CornellBox,
}
