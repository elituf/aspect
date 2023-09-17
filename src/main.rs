use num::integer::gcd;

mod args;

fn main() {
    let args: args::Args = argh::from_env();

    println!(
        "the aspect ratio of {} by {} is {}:{}",
        args.width,
        args.height,
        args.width / gcd(args.width, args.height),
        args.height / gcd(args.width, args.height)
    )
}
