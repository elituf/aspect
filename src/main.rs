use num::integer::gcd;

mod args;

fn calc_aspect(w: usize, h: usize) -> String {
    format!("{}:{}", w / gcd(w, h), h / gcd(w, h),)
}

fn calc_aspect_image(path: String) {
    match imagesize::size(&path) {
        Ok(size) => {
            let w = size.width;
            let h = size.height;
            println!(
                "File: {path}\nResolution: {w}x{h}\nAspect ratio: {}\n",
                calc_aspect(w, h),
            );
        }
        Err(why) => println!("{why}"),
    }
}

fn main() {
    let args: args::Args = argh::from_env();

    if args.width.is_some() && args.height.is_some() {
        let w = args.width.unwrap();
        let h = args.height.unwrap();
        println!("Resolution: {w}x{h}\nAspect ratio: {}\n", calc_aspect(w, h));
    }
    if args.image.is_some() {
        calc_aspect_image(args.image.unwrap());
    }
}
