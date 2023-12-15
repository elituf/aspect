use crate::gcd::gcd;
use eyre::Result;
use imagesize::ImageSize;
use std::{fmt, path::PathBuf};

pub struct Aspect {
    pub width: usize,
    pub height: usize,
}

pub struct Image {
    pub path: PathBuf,
    pub resolution: ImageSize,
    pub aspect: Aspect,
}

impl fmt::Display for Aspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.width, self.height)
    }
}

pub fn calc_aspect(w: usize, h: usize) -> Aspect {
    Aspect {
        width: w / gcd(w, h),
        height: h / gcd(w, h),
    }
}

pub fn calc_image_aspect(path: &PathBuf) -> Result<Image> {
    match imagesize::size(path) {
        Ok(resolution) => Ok(Image {
            path: path.to_path_buf(),
            resolution,
            aspect: calc_aspect(resolution.width, resolution.height),
        }),
        Err(why) => Err(why.into()),
    }
}
