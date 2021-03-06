use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(long, default_value_t = 500)]
    pub width: u32,

    #[clap(long, default_value_t = 500)]
    pub height: u32,

    #[clap(long)]
    pub preview: bool,

    #[clap(long, arg_enum, default_value_t = ArgCamera::ThinLens)]
    pub camera: ArgCamera,

    #[clap(long, default_value_t = 16)]
    pub samples: u8,
}

#[derive(ArgEnum, Debug, Clone)]
pub enum ArgCamera {
    Simple,
    ThinLens,
}
