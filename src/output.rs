/*
 * output.rs
 * author: ian brault <ian.brault@engineering.ucla.edu>
 */

use image::DynamicImage;

use crate::pixel::Pixel;


pub fn to_terminal(clusters: &Vec<Pixel>) {
    println!();
    for (i, color) in clusters.iter().enumerate() {
        println!("color {}: {}", i + 1, color.as_hex());
    }
}


pub fn to_image(filename: String, image: DynamicImage, clusters: Vec<Pixel>) {

}
