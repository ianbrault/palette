/*
 * output.rs
 * author: ian brault <ian.brault@engineering.ucla.edu>
 */

use std::cmp;

use image::{DynamicImage, GenericImageView, ImageBuffer, RgbaImage};

use crate::pixel::Pixel;


pub fn to_terminal(clusters: &[Pixel]) {
    println!();
    for (i, color) in clusters.iter().enumerate() {
        println!("color {}: {}", i + 1, color.as_hex());
    }
}


struct Image {
    image: DynamicImage,
    i_width: u32,
    i_height: u32,
    padding: u32,
    width: u32,
    height: u32,
    palette_padding: u32,
    palette_size: u32,
}

impl Image {
    fn new(image: DynamicImage, n: u32) -> Image {
        let i_width = image.width();
        let i_height = image.height();
        let padding = cmp::max(i_width, i_height) / 16;
        let palette_padding = padding / 2;
        let palette_size = (i_height - ((n - 1) * palette_padding)) / n;

        Image {
            image, i_width, i_height, padding,
            width: i_width + (3 * padding) + palette_size,
            height: i_height + (2 * padding),
            palette_padding, palette_size,
        }
    }

    fn is_image(&self, x: u32, y: u32) -> bool {
        let is_x = x >= self.padding && x < self.padding + self.i_width;
        let is_y = y >= self.padding && y < self.padding + self.i_height;
        is_x && is_y
    }

    // if yes, return the palette's index, otherwise return -1
    fn is_palette(&self, x: u32, y: u32, n_colors: u32) -> i32 {
        let base_x = self.padding + self.i_width + self.padding;
        // x is in the correct range
        let is_x = x >= base_x && x < base_x + self.palette_size;
        // y is not an outer border
        let is_y = y <= self.padding || y >= self.height - self.padding;
        if !is_x || is_y {
            return -1;

        }

        let mut iter_y = y as i32 - self.padding as i32;
        let mut palette_index: i32 = 0;
        while iter_y >= 0 {
            if iter_y < self.palette_size as i32 && palette_index < n_colors as i32 {
                return palette_index;
            }
            iter_y -= self.palette_size as i32;
            iter_y -= self.palette_padding as i32;
            palette_index += 1;
        }

        -1
    }
}


fn write_line(y: u32, imgbuf: &mut RgbaImage, image: &Image, clusters: &[Pixel]) {
    let n_clusters = clusters.len() as u32;

    for x in 0..image.width {
        if image.is_image(x, y) {
            let image_pixel = image.image.get_pixel(x - image.padding, y - image.padding);
            imgbuf.put_pixel(x, y, image_pixel);
        } else if image.is_palette(x, y, n_clusters) >= 0 {
            let pi = image.is_palette(x, y, n_clusters) as usize;
            let pixel = clusters[pi].as_rgba();
            imgbuf.put_pixel(x, y, pixel);
        } else {
            imgbuf.put_pixel(x, y, image::Rgba([255, 255, 255, 1]));
        }
    }
}

pub fn to_image(filename: String, image: DynamicImage, clusters: Vec<Pixel>) {
    let image = Image::new(image, clusters.len() as u32);

    let mut imgbuf = ImageBuffer::new(image.width, image.height);
    for y in 0..image.height {
        write_line(y, &mut imgbuf, &image, &clusters);
    }

    imgbuf.save(filename).unwrap();
}
