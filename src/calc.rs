use crate::gcd::gcd;
use eyre::Result;
use imagesize::ImageSize;
use std::path::PathBuf;

pub fn calc_aspect(w: usize, h: usize) -> String {
    format!("{}:{}", w / gcd(w, h), h / gcd(w, h))
}

pub fn calc_image_aspect(path: &PathBuf) -> Result<String> {
    match imagesize::size(path) {
        Ok(ImageSize { width, height }) => Ok(format!(
            "File: {}\nResolution: {width}x{height}\nAspect ratio: {}",
            path.display(),
            calc_aspect(width, height),
        )),
        Err(why) => Err(why.into()),
    }
}
