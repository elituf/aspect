use argh::FromArgs;

/// aspect: an aspect ratio calculator
#[derive(FromArgs, PartialEq, Eq)]
pub struct Args {
    /// the width of your screen or image
    #[argh(option, short = 'w')]
    pub width: Option<i32>,

    /// the height of your screen or image
    #[argh(option, short = 'h')]
    pub height: Option<i32>,

    /// image
    #[argh(option, short = 'i')]
    pub image: Option<String>,
}
