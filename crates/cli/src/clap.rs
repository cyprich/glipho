use clap::Parser;

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

#[derive(Parser, Debug)]
#[command(
    about = "Rusty Fotos - Image manipulation software to make glitchy effects",
    long_about = "Rusty Fotos - Image manipulation software to make glitchy effects
You can either run via interactive mode (-i) or by specifying Image file (-f) and Effects file (-e)
Effects file can be created in interactive mode"
)]
pub(crate) struct Cli {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub interactive: Option<bool>,

    #[arg(short = 'f', long)]
    pub image_file: Option<String>,

    #[arg(short, long)]
    pub effects_file: Option<String>,
    // steps: Option<Vec<Step>>,
}
