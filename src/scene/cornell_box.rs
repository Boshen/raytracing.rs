use std::sync::Arc;

use crate::accelerator::Bvh;
use crate::args::{ArgCamera, Args};
use crate::asset::Asset;
use crate::brdf::{GlossySpecular, Lambertian, PerfectSpecular};
use crate::camera::{Camera, Pinhole, Setting, ThinLens};
use crate::color::Color;
use crate::geometric_object::{Geometry, Sphere};
use crate::light::{Ambient, AmbientOcculuder, Light};
use crate::material::{Phong, Reflective};
use crate::model::{Pot3, Vec3};
use crate::ray::{HitRecord, Ray};

pub struct CornellBox {
    pub view_width: u32,
    pub view_height: u32,
    pub camera: Box<dyn Camera>,
    pub ambient_light: Arc<Ambient>,
    pub lights: Vec<Arc<dyn Light>>,
    pub root: Vec<Arc<dyn Geometry>>,
}

impl CornellBox {
    #[must_use]
    pub fn new(view_width: u32, view_height: u32, args: &Args) -> Self {
        let scale = 555.0;
        let mut asset = Asset::new("./assets/cornell_box.obj", scale);

        let ambient_light = Arc::new(Ambient {
            ls: 0.1,
            cl: Vec3::repeat(1.0),
        });

        let ambient_occuluder = Arc::new(AmbientOcculuder::new(1.0, Vec3::repeat(1.0)));
        asset.lights.push(ambient_occuluder);

        let mut camera_setting =
            Setting::new(Pot3::new(0.0, 0.0, -3.0), Pot3::new(0.0, 0.0, 0.0), 520.0);
        camera_setting.set_view((view_width, view_height));

        let camera: Box<dyn Camera> = match args.camera {
            ArgCamera::Simple => Box::new(Pinhole::new(camera_setting)),
            ArgCamera::ThinLens => Box::new(ThinLens::new(camera_setting, 0.001, 510.0)),
        };

        // add ball with reflective
        let ball1_material = Reflective::new(
            Lambertian::new(0.1, Color::repeat(1.0)),
            Lambertian::new(0.7, Color::repeat(1.0)),
            GlossySpecular::new(0.1, 0.0, Color::repeat(1.0)),
            PerfectSpecular::new(0.8, Color::repeat(1.0)),
        );
        let ball1 = Arc::new(Sphere::new(
            ball1_material,
            40.0,
            Pot3::new(450.0, 40.0, 450.0),
            scale,
        ));
        asset.geometries.push(ball1);

        // add ball with phong
        let ball2_material = Phong::new(
            Lambertian::new(0.1, Color::repeat(1.0)),
            Lambertian::new(0.1, Color::repeat(1.0)),
            GlossySpecular::new(0.3, 2.0, Color::repeat(1.0)),
        );
        let ball2 = Arc::new(Sphere::new(
            ball2_material,
            30.0,
            Pot3::new(350.0, 30.0, 500.0),
            scale,
        ));
        asset.geometries.push(ball2);

        let root = vec![Bvh::construct(asset.geometries)];

        Self {
            view_width,
            view_height,
            camera,
            ambient_light,
            lights: asset.lights,
            root,
        }
    }

    /// # Panics
    /// will panic if `partial_cmp` fails
    #[must_use]
    pub fn intersects(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.root
            .iter()
            .filter_map(|o| o.intersects(ray, tmin, tmax))
            .min_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap())
    }
}
