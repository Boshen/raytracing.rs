//! Cornell Box scene implementation.
//!
//! The Cornell Box is a classic test scene in computer graphics,
//! originally created at Cornell University to test global illumination
//! algorithms. It consists of a box with colored walls, light source,
//! and objects to demonstrate light transport.

use std::sync::Arc;

use crate::{
    accelerator::Bvh,
    args::{ArgCamera, Args},
    asset::Asset,
    brdf::{GlossySpecular, Lambertian, PerfectSpecular},
    camera::{Camera, Pinhole, Setting, ThinLens},
    color::Color,
    geometric_object::{Geometry, Sphere},
    light::{Ambient, AmbientOcculuder, Light},
    material::{Phong, Reflective},
    model::{Pot3, Vec3},
    ray::{HitRecord, Ray},
};

use super::Scene;

/// Classic Cornell Box scene with colored walls and spheres.
///
/// Features:
/// - Red left wall, green right wall, white other walls
/// - Area light at the top
/// - Two spheres with different materials (reflective and Phong)
/// - BVH acceleration structure
pub struct CornellBox {
    pub view_width: u32,
    pub view_height: u32,
    pub camera: Box<dyn Camera>,
    pub ambient_light: Arc<Ambient>,
    pub lights: Vec<Arc<dyn Light>>,
    pub root: Vec<Arc<dyn Geometry>>,
}

impl CornellBox {
    /// Creates a new Cornell Box scene.
    ///
    /// # Arguments
    /// * `view_width` - Width of the output image in pixels
    /// * `view_height` - Height of the output image in pixels
    /// * `args` - Command-line arguments for configuration
    ///
    /// # Scene Setup
    /// 1. Loads the Cornell Box geometry from OBJ file
    /// 2. Adds two spheres with different materials
    /// 3. Sets up camera (pinhole or thin lens)
    /// 4. Configures lighting (ambient + area light)
    /// 5. Builds BVH acceleration structure
    #[must_use]
    pub fn new(view_width: u32, view_height: u32, args: &Args) -> Self {
        // Scene scale factor (Cornell Box is traditionally 555 units)
        let scale = 555.0;

        // Load base geometry from OBJ file
        let mut asset = Asset::new("./assets/cornell_box.obj", scale);

        // Configure ambient lighting
        let ambient_light = Arc::new(Ambient { ls: 0.1, cl: Vec3::repeat(1.0) });

        // Add area light at the top (simulates the ceiling light)
        let ambient_occuluder = Arc::new(AmbientOcculuder::new(1.0, Vec3::repeat(1.0)));
        asset.lights.push(ambient_occuluder);

        // Configure camera position and field of view
        let mut camera_setting = Setting::new(
            Pot3::new(0.0, 0.0, -3.0), // Eye position
            Pot3::new(0.0, 0.0, 0.0),  // Look-at point
            520.0,                     // Focal length
        );
        camera_setting.set_view((view_width, view_height));

        // Select camera type based on arguments
        let camera: Box<dyn Camera> = match args.camera {
            ArgCamera::Simple => Box::new(Pinhole::new(camera_setting)),
            ArgCamera::ThinLens => Box::new(ThinLens::new(
                camera_setting,
                0.001, // Lens radius (aperture)
                510.0, // Focus distance
            )),
        };

        // Add reflective sphere (demonstrates perfect and glossy reflections)
        let ball1_material = Reflective::new(
            Lambertian::new(0.1, Color::repeat(1.0)),          // Ambient
            Lambertian::new(0.7, Color::repeat(1.0)),          // Diffuse
            GlossySpecular::new(0.1, 0.0, Color::repeat(1.0)), // Specular
            PerfectSpecular::new(0.8, Color::repeat(1.0)),     // Reflection
        );
        let ball1 = Arc::new(Sphere::new(
            ball1_material,
            40.0,                          // Radius
            Pot3::new(450.0, 40.0, 450.0), // Position (right side, on floor)
            scale,
        ));
        asset.geometries.push(ball1);

        // Add Phong-shaded sphere (demonstrates classic shading model)
        let ball2_material = Phong::new(
            Lambertian::new(0.1, Color::repeat(1.0)),          // Ambient
            Lambertian::new(0.1, Color::repeat(1.0)),          // Diffuse
            GlossySpecular::new(0.3, 2.0, Color::repeat(1.0)), // Specular with shininess
        );
        let ball2 = Arc::new(Sphere::new(
            ball2_material,
            30.0,                          // Radius
            Pot3::new(350.0, 30.0, 500.0), // Position (left side, on floor)
            scale,
        ));
        asset.geometries.push(ball2);

        // Build BVH acceleration structure for efficient ray tracing
        let root = vec![Bvh::construct(asset.geometries)];

        Self { view_width, view_height, camera, ambient_light, lights: asset.lights, root }
    }

    /// Implementation moved from trait method for performance.
    /// Tests for ray-object intersection in the scene.
    ///
    /// # Panics
    /// Will panic if `partial_cmp` fails (shouldn't happen with valid geometry)
    #[must_use]
    pub fn intersects(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord<'_>> {
        self.root
            .iter()
            .filter_map(|o| o.intersects(ray, tmin, tmax))
            .min_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap())
    }
}

impl Scene for CornellBox {
    fn view_width(&self) -> u32 {
        self.view_width
    }

    fn view_height(&self) -> u32 {
        self.view_height
    }

    fn camera(&self) -> &dyn Camera {
        self.camera.as_ref()
    }

    fn ambient_light(&self) -> &Arc<Ambient> {
        &self.ambient_light
    }

    fn lights(&self) -> &[Arc<dyn Light>] {
        &self.lights
    }

    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        self.intersects(ray, t_min, t_max)
    }
}
