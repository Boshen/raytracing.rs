use crate::args::ArgCamera;
use crate::camera::{Camera, Setting, Simple, ThinLens};
use crate::light::{Ambient, AmbientOcculuder};
use crate::model::{Pot3, Vec3};
use std::sync::Arc;

pub struct CornellBox {
    pub view_width: u32,
    pub view_height: u32,
    pub camera_setting: Setting,
    pub ambient_light: Arc<Ambient>,
    pub ambient_occuluder: Arc<AmbientOcculuder>,
}

impl Default for CornellBox {
    fn default() -> Self {
        let ambient_light = Arc::new(Ambient {
            ls: 0.1,
            cl: Vec3::repeat(1.0),
        });
        let ambient_occuluder = Arc::new(AmbientOcculuder::new(1.0, Vec3::repeat(1.0)));
        let camera_setting =
            Setting::new(Pot3::new(0.0, 0.0, -3.0), Pot3::new(0.0, 0.0, 0.0), 500.0);
        Self {
            view_width: 500,
            view_height: 500,
            camera_setting,
            ambient_light,
            ambient_occuluder,
        }
    }
}

impl CornellBox {
    #[must_use]
    pub fn camera(self, camera: &ArgCamera) -> Box<dyn Camera> {
        match camera {
            ArgCamera::Simple => Box::new(Simple::new(self.camera_setting)),
            ArgCamera::ThinLens => Box::new(ThinLens::new(self.camera_setting, 0.001, 500.0)),
        }
    }
}
