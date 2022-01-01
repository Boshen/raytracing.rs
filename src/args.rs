use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(long)]
    pub preview: bool,

    #[clap(long, arg_enum, default_value_t = ArgCamera::ThinLens)]
    pub camera: ArgCamera,
}

#[derive(ArgEnum, Debug, Clone)]
pub enum ArgCamera {
    Simple,
    ThinLens,
}
