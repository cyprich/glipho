use clap::Parser;
use lib::Step;

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub interactive: Option<bool>,
    // steps: Option<Vec<Step>>,
}
