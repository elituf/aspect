use args::Args;
use clap::Parser;
use colored::Colorize;
use imagesize::ImageError;

mod args;
mod calc;
mod gcd;
mod tests;

fn main() -> Result<(), ImageError> {
    let args = Args::parse();
    if let (Some(w), Some(h)) = (args.width, args.height) {
        println!(
            "{}: {w}x{h}\n{}: {}",
            "Resolution".green(),
            "Aspect ratio".green(),
            calc::aspect(w, h)
        );
    }
    if args.width.is_some() && args.height.is_some() && args.image.is_some() {
        println!(); // print an empty line should the user use all the arguments
    }
    if let Some(img) = args.image {
        let image = calc::image_aspect(&img)?;
        println!(
            "{}: {}\n{}: {}x{}\n{}: {}",
            "File".blue(),
            image.path.display(),
            "Resolution".blue(),
            image.resolution.width,
            image.resolution.height,
            "Aspect ratio".blue(),
            image.aspect,
        );
    }
    Ok(())
}
