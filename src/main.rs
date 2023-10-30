use crate::args::Args;
use clap::Parser;
use eyre::Result;
use gcd::Gcd;
use std::path::PathBuf;

mod args;

fn calc_aspect(w: usize, h: usize) -> String {
    format!("{}:{}", w / w.gcd(h), h / w.gcd(h),)
}

fn calc_aspect_image(path: &PathBuf) -> Result<String> {
    match imagesize::size(path) {
        Ok(size) => {
            let w = size.width;
            let h = size.height;
            Ok(format!(
                "File: {}\nResolution: {w}x{h}\nAspect ratio: {}",
                path.canonicalize()?.to_string_lossy().trim_start_matches(r"\\?\"),
                calc_aspect(w, h),
            ))
        }
        Err(why) => Err(why.into()),
    }
}

fn main() {
    let args = Args::parse();

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
