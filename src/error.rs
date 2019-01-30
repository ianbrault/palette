/*
 * src/main.rs
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use image::{ColorType, ImageError};
use colored::*;


pub fn on_err(err_msg: &str) {
    eprintln!("{} {}", "error:".red().bold(), err_msg);
}


fn color_type_to_string(ctype: ColorType) -> &'static str {
    match ctype {
        ColorType::Gray(_) => "grayscale",
        ColorType::RGB(_) => "RGB",
        ColorType::Palette(_) => "color palette index",
        ColorType::GrayA(_) => "grayscale (alpha)",
        ColorType::RGBA(_) => "RGBA",
        ColorType::BGR(_) => "BGR",
        ColorType::BGRA(_) => "BGRA",
    }
}


pub fn on_image_err(img_err: ImageError, image: String) {
    let err_msg = match img_err {
        ImageError::FormatError(fe) =>
            format!("{} is not formatted properly: {}", image, fe),
        ImageError::DimensionError =>
            format!("{} is either too small or too large to process", image),
        ImageError::UnsupportedError(format) =>
            format!("the \"{}\" format is not supported", format),
        ImageError::UnsupportedColor(ctype) =>
            format!("the \"{}\" color type is not supported", color_type_to_string(ctype)),
        ImageError::NotEnoughData => String::from("not enough data to decode the image"),
        ImageError::IoError(err) => format!("{}: {}", image, err),
        ImageError::ImageEnd => String::from("the end of the image was reached"),
        ImageError::InsufficientMemory => String::from("the program ran out of memory"),
    };
    on_err(err_msg.as_str());
}
