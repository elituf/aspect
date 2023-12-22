use crate::gcd::gcd;
use imagesize::{ImageError, ImageSize};
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

pub fn aspect(w: usize, h: usize) -> Aspect {
    Aspect {
        width: w / gcd(w, h),
        height: h / gcd(w, h),
    }
}

pub fn image_aspect(path: &PathBuf) -> Result<Image, ImageError> {
    match imagesize::size(path) {
        Ok(resolution) => Ok(Image {
            path: path.clone(),
            resolution,
            aspect: aspect(resolution.width, resolution.height),
        }),
        Err(why) => Err(why),
    }
}
