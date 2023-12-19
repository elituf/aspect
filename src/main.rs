use args::Args;
use calc::{calc_aspect, calc_image_aspect};
use clap::Parser;
use colored::Colorize;
use imagesize::ImageError;

mod args;
mod calc;
mod gcd;

fn main() -> Result<(), ImageError> {
    let args = Args::parse();

    if let (Some(w), Some(h)) = (args.width, args.height) {
        println!(
            "{}: {w}x{h}\n{}: {}",
            format!("Resolution").green(),
            format!("Aspect ratio").green(),
            calc_aspect(w, h)
        );
    }
    if args.width.is_some() && args.height.is_some() && args.image.is_some() {
        println!(); // print an empty line should the user use all the arguments
    }
    if let Some(img) = args.image {
        let image = calc_image_aspect(&img)?;
        println!(
            "{}\n{}\n{}",
            format!("{}: {}", format!("File").blue(), image.path.display()),
            format!("{}: {}x{}", format!("Resolution").blue(), image.resolution.width, image.resolution.height,),
            format!("{}: {}", format!("Aspect ratio").blue(), image.aspect),
        );
    }

    Ok(())
}
