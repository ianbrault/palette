/*
 * src/main.rs
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

mod error;
mod kmeans;
mod pixel;

use std::cmp;
use std::fs;

use clap::{App, Arg};

use crate::pixel::Pixel;

// Program Configuration & Command-Line Args

struct Config {
    image_file: String,
}

impl Config {
    fn new(args: clap::ArgMatches) -> Config {
        Config {
            image_file: String::from(args.value_of("image").unwrap()),
        }
    }

    fn parse() -> Config {
        let version = format!("v{}", env!("CARGO_PKG_VERSION"));
        let args = App::new(env!("CARGO_PKG_NAME"))
            .version(version.as_str())
            .author(env!("CARGO_PKG_AUTHORS"))
            .arg(Arg::with_name("image")
                .required(true)
                .help("input image file"));

        Config::new(args.get_matches())
    }
}


fn get_bytestring(nbytes: u64) -> String {
    let power = (nbytes as f64).log10().floor() as u64;
    let unit_power = cmp::min(power - (power % 3), 9) as u32;
    let unit_str = match unit_power {
        0 => "B",
        3 => "kB",
        6 => "MB",
        _ => "GB",
    };

    let formatted_bytes = nbytes as f64 / 10_u64.pow(unit_power) as f64;
    format!("{:.3} {}", formatted_bytes, unit_str)
}


fn get_pixels(image: image::DynamicImage) -> Vec<Pixel> {
    // reads as RGB, ignores alpha channel
    let image_rgb = image.to_rgb();

    // initialize buffer based on size hint
    let size_hint = image_rgb.pixels().size_hint();
    let buf_size = if size_hint.1.is_some() {
        size_hint.1.unwrap()
    } else {
        size_hint.0
    };

    let mut pixel_buf: Vec<Pixel> = Vec::with_capacity(buf_size);
    for pixel in image_rgb.pixels() {
        let [r, g, b] = pixel.data;
        pixel_buf.push(Pixel::new(r, g, b));
    }

    pixel_buf
}


fn generate_palette(cfg: Config, image: image::DynamicImage) {
    let image_size = fs::metadata(&cfg.image_file).unwrap().len();
    println!("using image {} ({})", &cfg.image_file, get_bytestring(image_size));
    println!("loading image...");

    // load pixel values into memory
    let pixel_buf = get_pixels(image);

    // run k-means clustering to get palette values as clusters
    let clusters = kmeans::k_cluster(5, pixel_buf);
}


fn main() {
    let cfg = Config::parse();

    match image::open(&cfg.image_file) {
        image @ Ok(_) => generate_palette(cfg, image.unwrap()),
        Err(image_err) => error::on_image_err(image_err, cfg.image_file),
    }
}
