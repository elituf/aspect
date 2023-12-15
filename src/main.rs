use args::Args;
use calc::{calc_aspect, calc_image_aspect};
use clap::Parser;
use eyre::Result;

mod args;
mod calc;
mod gcd;

fn main() -> Result<()> {
    let args = Args::parse();

    if let (Some(w), Some(h)) = (args.width, args.height) {
        println!("Resolution: {w}x{h}\nAspect ratio: {}", calc_aspect(w, h));
    }
    if args.width.is_some() && args.height.is_some() && args.image.is_some() {
        println!(); // print an empty line should the user use all the arguments
    }
    if let Some(img) = args.image {
        let image = calc_image_aspect(&img)?;
        println!(
            "File: {}\nResolution: {}x{}\nAspect ratio: {}",
            image.path.display(),
            image.resolution.width,
            image.resolution.height,
            image.aspect
        );
    }

    Ok(())
}
