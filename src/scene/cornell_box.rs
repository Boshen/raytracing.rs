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
    config::{
        camera::{DEFAULT_EYE_POSITION, DEFAULT_FOCAL_DISTANCE, DEFAULT_FOCAL_LENGTH, DEFAULT_LENS_RADIUS, DEFAULT_LOOKAT_POSITION},
        geometry::spheres::{LARGE_SPHERE_POSITION, LARGE_SPHERE_RADIUS, SMALL_SPHERE_POSITION, SMALL_SPHERE_RADIUS},
        scene::{CORNELL_BOX_ASSET_PATH, CORNELL_BOX_SCALE, DEFAULT_AMBIENT_STRENGTH},
    },
    error::Result,
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
    ///
    /// # Errors
    /// Returns an error if the scene assets cannot be loaded
    pub fn new(view_width: u32, view_height: u32, args: &Args) -> Result<Self> {
        // Load base geometry from OBJ file
        let mut asset = Asset::new(CORNELL_BOX_ASSET_PATH, CORNELL_BOX_SCALE)?;

        // Configure ambient lighting
        let ambient_light = Arc::new(Ambient { ls: DEFAULT_AMBIENT_STRENGTH, cl: Vec3::repeat(1.0) });

        // Add area light at the top (simulates the ceiling light)
        let ambient_occuluder = Arc::new(AmbientOcculuder::new(1.0, Vec3::repeat(1.0)));
        asset.lights.push(ambient_occuluder);

        // Configure camera position and field of view
        let mut camera_setting = Setting::new(
            Pot3::new(DEFAULT_EYE_POSITION[0], DEFAULT_EYE_POSITION[1], DEFAULT_EYE_POSITION[2]),
            Pot3::new(DEFAULT_LOOKAT_POSITION[0], DEFAULT_LOOKAT_POSITION[1], DEFAULT_LOOKAT_POSITION[2]),
            DEFAULT_FOCAL_LENGTH,
        );
        camera_setting.set_view((view_width, view_height));

        // Select camera type based on arguments
        let camera: Box<dyn Camera> = match args.camera {
            ArgCamera::Simple => Box::new(Pinhole::new(camera_setting)),
            ArgCamera::ThinLens => Box::new(ThinLens::new(
                camera_setting,
                DEFAULT_LENS_RADIUS,
                DEFAULT_FOCAL_DISTANCE,
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
            LARGE_SPHERE_RADIUS,
            Pot3::new(LARGE_SPHERE_POSITION[0], LARGE_SPHERE_POSITION[1], LARGE_SPHERE_POSITION[2]),
            CORNELL_BOX_SCALE,
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
            SMALL_SPHERE_RADIUS,
            Pot3::new(SMALL_SPHERE_POSITION[0], SMALL_SPHERE_POSITION[1], SMALL_SPHERE_POSITION[2]),
            CORNELL_BOX_SCALE,
        ));
        asset.geometries.push(ball2);

        // Build BVH acceleration structure for efficient ray tracing
        let root = vec![Bvh::construct(asset.geometries)];

        Ok(Self { view_width, view_height, camera, ambient_light, lights: asset.lights, root })
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
