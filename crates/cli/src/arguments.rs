use clap::Parser;

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

#[derive(Parser, Debug)]
#[command(
    about = "Rusty Fotos - Image manipulation software to make glitchy effects",
    long_about = "Rusty Fotos - Image manipulation software to make glitchy effects
You can either run via manual mode (-m) or by specifying input image (-i), output image (-o) and effects file (-e)
Effects file can be created in interactive mode"
)]
pub(crate) struct Arguments {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub manual: Option<bool>,

    #[arg(short, long)]
    pub input: Option<String>,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long)]
    pub effects: Option<String>,
}
