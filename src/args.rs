use clap::Parser;
use std::path::PathBuf;

/// aspect: an aspect ratio calculator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    /// specify width to use for aspect ratio calculation
    #[arg(short = 'W', long)]
    pub width: Option<usize>,

    /// specify height to use for aspect ratio calculation
    #[arg(short = 'H', long)]
    pub height: Option<usize>,

    /// get the dimensions and aspect ratio of an image
    #[arg(short, long)]
    pub image: Option<PathBuf>,
}
