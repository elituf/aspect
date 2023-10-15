use argh::FromArgs;

/// aspect: an aspect ratio calculator
#[derive(FromArgs, PartialEq, Eq)]
pub struct Args {
    /// the width to calculate with
    #[argh(option, short = 'w')]
    pub width: Option<i32>,

    /// the height to calculate with
    #[argh(option, short = 'h')]
    pub height: Option<i32>,

    /// get the size and aspect of an image
    #[argh(option, short = 'i')]
    pub image: Option<String>,
}
