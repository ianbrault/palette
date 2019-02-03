/*
 * pixel.rs
 * pixel data structure implementation
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use crate::kmeans::KVector;

#[derive(Clone, Debug, PartialEq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }
}

// for use in k-means clustering
impl KVector for Pixel {
    fn distance(&self, other: &Pixel) -> u32 {
        // cast up to i64 to avoid unsigned overflow
        let r_dist = (self.r as i64) - (other.r as i64);
        let g_dist = (self.g as i64) - (other.g as i64);
        let b_dist = (self.b as i64) - (other.b as i64);
        (r_dist*r_dist + g_dist*g_dist + b_dist*b_dist) as u32
    }

    fn average(vectors: &Vec<Pixel>) -> Pixel {
        let sum: (u64, u64, u64) = (0, 0, 0);
        let sum = vectors.iter()
            .fold(sum, |acc, p| (acc.0 + (p.r as u64), acc.1 + (p.g as u64), acc.2 + (p.b as u64)));

        let n_vecs = vectors.len() as u64;
        Pixel::new((sum.0 / n_vecs) as u8, (sum.1 / n_vecs) as u8, (sum.2 / n_vecs) as u8)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p = Pixel::new(12, 4, 20);
        assert_eq!(p.distance(&Pixel::new(8, 5, 18)), 21);
        assert_eq!(p.distance(&Pixel::new(16, 3, 22)), 21);
        assert_eq!(p.distance(&Pixel::new(12, 4, 20)), 0);
    }

    #[test]
    fn test_average() {
        let data = vec![
            Pixel::new(0,0,0), Pixel::new(4,5,6), Pixel::new(12,8,20), Pixel::new(16,1,42)
        ];
        assert_eq!(Pixel::average(&data), Pixel::new(8, 3, 17));
    }
}
