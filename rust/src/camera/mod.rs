use enum_dispatch::enum_dispatch;

pub mod simple;

use crate::world::World;
pub use simple::*;

#[enum_dispatch]
pub enum CameraEnum {
    SimpleCamera,
}

#[enum_dispatch(CameraEnum)]
pub trait Camera {
    fn render_scene(&self, world: &World) -> Vec<u8>;
}
