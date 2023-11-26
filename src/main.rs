use args::Args;
use clap::Parser;
use eyre::Result;
use gcd::gcd;
use imagesize::ImageSize;
use std::path::PathBuf;

mod args;
mod gcd;

fn calc_aspect(w: usize, h: usize) -> String {
    format!("{}:{}", w / gcd(w, h), h / gcd(w, h),)
}

fn calc_aspect_image(path: &PathBuf) -> Result<String> {
    match imagesize::size(path) {
        Ok(ImageSize { width, height }) => Ok(format!(
            "File: {}\nResolution: {width}x{height}\nAspect ratio: {}",
            path.display(),
            calc_aspect(width, height),
        )),
        Err(why) => Err(why.into()),
    }
}

fn main() {
    let args = Args::parse();

    if let (Some(w), Some(h)) = (args.width, args.height) {
        println!("Resolution: {w}x{h}\nAspect ratio: {}", calc_aspect(w, h));
    }
    if args.width.is_some() && args.height.is_some() && args.image.is_some() {
        println!(); // print an empty line should the user use all the arguments
    }
    if let Some(img) = args.image {
        match calc_aspect_image(&img) {
            Ok(result) => println!("{result}"),
            Err(why) => println!("{why}"),
        }
    }
}
