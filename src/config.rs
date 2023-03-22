use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "action-validator",
    about = "A validator for GitHub Action and Workflow YAML files"
)]
pub struct Config {
    /// Be more verbose
    #[arg(short, long)]
    pub verbose: bool,

    /// Input file
    #[arg(long)]
    pub src: PathBuf,
}
