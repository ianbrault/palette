/*
 * src/main.rs
 * author: ian brault <ian.brault@engineering.ucla.edu>
 */

mod config;
mod error;
mod kmeans;
mod output;
mod pixel;

use std::cmp;
use std::fs;

use image::DynamicImage;

use crate::config::Config;
use crate::pixel::Pixel;


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


fn get_pixels(image: &DynamicImage) -> Vec<Pixel> {
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


fn generate_palette(cfg: Config, image: DynamicImage) {
    let image_size = fs::metadata(&cfg.input_file).unwrap().len();
    println!("using image {} ({})", &cfg.input_file, get_bytestring(image_size));
    println!("loading image...");

    // load pixel values into memory
    let pixel_buf = get_pixels(&image);

    println!("analyzing colors...");
    // run k-means clustering to get palette values as clusters
    let clusters = kmeans::k_cluster(cfg.n_colors as u32, pixel_buf);

    if cfg.term_output {
        output::to_terminal(&clusters);
    }

    if cfg.image_output {
        output::to_image(cfg.output_file, image, clusters);
    }
}


fn main() {
    let cfg = Config::parse();

    match image::open(&cfg.input_file) {
        image @ Ok(_) => generate_palette(cfg, image.unwrap()),
        Err(image_err) => error::on_image_err(image_err, cfg.input_file),
    }
}
