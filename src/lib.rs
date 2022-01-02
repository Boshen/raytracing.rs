#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::many_single_char_names
)]

pub mod aabb;
pub mod args;
pub mod asset;
pub mod brdf;
pub mod camera;
pub mod color;
pub mod geometric_object;
pub mod light;
pub mod material;
pub mod model;
pub mod ray;
pub mod renderer;
pub mod sampler;
pub mod scene;
