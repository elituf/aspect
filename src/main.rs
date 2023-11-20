mod args;
mod gcd;

use clap::Parser;
use eyre::Result;
use gcd::gcd;
use std::path::PathBuf;

fn calc_aspect(w: usize, h: usize) -> String {
    format!("{}:{}", w / gcd(w, h), h / gcd(w, h),)
}

fn calc_aspect_image(path: &PathBuf) -> Result<String> {
    match imagesize::size(path) {
        Ok(size) => Ok(format!(
            "File: {}\nResolution: {}x{}\nAspect ratio: {}",
            path.canonicalize()?.to_string_lossy().trim_start_matches(r"\\?\"),
            size.width,
            size.height,
            calc_aspect(size.width, size.height),
        )),
        Err(why) => Err(why.into()),
    }
}

fn main() {
    let args = args::Args::parse();

    if let (Some(w), Some(h)) = (args.width, args.height) {
        println!("Resolution: {w}x{h}\nAspect ratio: {}", calc_aspect(w, h));
    }
    if args.width.is_some() && args.height.is_some() && args.image.is_some() {
        println!();
    }
    if let Some(img) = args.image {
        match calc_aspect_image(&img) {
            Ok(result) => println!("{result}"),
            Err(why) => println!("{why}"),
        }
    }
}
