use crate::model::{Pot3, Vec3};

pub struct Setting {
    pub up: Vec3,
    pub eye: Pot3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub view_plane_distance: f64,
    pub sample_points_sqrt: u8,
    pub view_width: u32,
    pub view_height: u32,
    pub pixel_size: f64,
}

impl Setting {
    #[must_use]
    pub fn new(eye: Pot3, lookat: Pot3, view_plane_distance: f64) -> Self {
        let up = Vec3::new(0.0, 1.0, 0.0);
        let w = (eye - lookat).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u).normalize();
        Self {
            eye,
            w,
            u,
            v,
            up,
            view_plane_distance,
            sample_points_sqrt: 1,
            view_width: 100,
            view_height: 100,
            pixel_size: 1.0,
        }
    }

    pub fn set_sample_points_sqrt(&mut self, sample_points_sqrt: u8) {
        self.sample_points_sqrt = sample_points_sqrt;
    }

    pub fn set_view(&mut self, (view_width, view_height): (u32, u32)) {
        self.view_width = view_width;
        self.view_height = view_height;
    }
}
