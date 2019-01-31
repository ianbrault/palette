/*
 * pixel.rs
 * pixel data structure implementation
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use rand::random;

use crate::kmeans::KVector;

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }

    // for use in k-means clustering
    pub fn generate_centroids(k: usize) -> Vec<Pixel> {
        let mut centroids = Vec::<Pixel>::with_capacity(k);
        for _ in 0..k {
            centroids.push(Pixel::new(random::<u8>(), random::<u8>(), random::<u8>()));
        };
        centroids
    }
}

// for use in k-means clustering
impl KVector for Pixel {
    fn distance(&self, other: Pixel) -> u64 {
        let r_dist = ((self.r - other.r) as u16) * ((self.r - other.r) as u16);
        let g_dist = ((self.g - other.g) as u16) * ((self.g - other.g) as u16);
        let b_dist = ((self.b - other.b) as u16) * ((self.b - other.b) as u16);
        ((r_dist + g_dist + b_dist) as f64).sqrt() as u64
    }

    fn sq_average(vectors: Vec<Pixel>) -> Pixel {
        let mut sum: (u64, u64, u64) = (0, 0, 0);
        let n_vecs = vectors.len() as u64;
        for vector in vectors {
            sum.0 += vector.r as u64;
            sum.1 += vector.g as u64;
            sum.2 += vector.b as u64;
        }

        let r = (sum.0 / n_vecs) as u8;
        let g = (sum.1 / n_vecs) as u8;
        let b = (sum.2 / n_vecs) as u8;
        Pixel::new(r, g, b)
    }
}
