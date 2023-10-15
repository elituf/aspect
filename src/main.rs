#[allow(unused)]
use colored::Colorize;
use num::integer::gcd;

mod args;

// you may ask, "oh why do we need so many functions?!@1!@!>?>/"
// shut the helll UP
fn calc_aspect(w: usize, h: usize) -> String {
    format!("{}:{}", w / gcd(w, h), h / gcd(w, h),)
}

fn calc_aspect_res(w: usize, h: usize) {
    println!(
        "Resolution: {}\nAspect ratio: {}\n",
        format!("{w}x{h}"),
        calc_aspect(w, h),
    );
}

fn calc_aspect_image(path: String) {
    match imagesize::size(&path) {
        Ok(size) => {
            let w = size.width;
            let h = size.height;
            println!(
                "File: {}\nResolution: {}\nAspect ratio: {}\n",
                format!("{path}"),
                format!("{w}x{h}"),
                calc_aspect(w, h),
            );
        }
        Err(why) => println!("{why}"),
    }
}

fn main() {
    let args: args::Args = argh::from_env();

    if args.width.is_some() && args.height.is_some() {
        calc_aspect_res(args.width.unwrap(), args.height.unwrap());
    }

    if args.image.is_some() {
        calc_aspect_image(args.image.unwrap());
    }
}
