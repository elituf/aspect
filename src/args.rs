use argh::FromArgs;

/// aspect: an aspect ratio calculator
#[derive(FromArgs, PartialEq)]
pub struct Args {
    /// the width of your screen or image
    #[argh(option, short = 'w')]
    pub width: i32,

    /// the height of your screen or image
    #[argh(option, short = 'h')]
    pub height: i32,
}
