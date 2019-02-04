/*
 * pixel.rs
 * pixel data structure implementation
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use std::ops::Mul;

use crate::kmeans::GenericVector;

#[derive(Clone, Debug, Default, PartialEq)]
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

impl Mul<f64> for Pixel {
    type Output = Pixel;

    fn mul(self, rhs: f64) -> Pixel {
        Pixel {
            r: ((self.r as f64) * rhs) as u8,
            g: ((self.g as f64) * rhs) as u8,
            b: ((self.b as f64) * rhs) as u8,
        }
    }
}

impl GenericVector for Pixel {
    fn average(vectors: Vec<&Pixel>) -> Pixel {
        let n = vectors.len() as u64;

        let mut sum: (u64, u64, u64) = (0, 0, 0);
        for v in vectors {
            sum.0 += v.r as u64;
            sum.1 += v.g as u64;
            sum.2 += v.b as u64;
        }

        Pixel::new(sum.0 as u8, sum.1 as u8, sum.2 as u8) * (1.0 / n as f64)
    }

    fn distance(&self, other: &Pixel) -> u32 {
        // cast up to i64 to avoid unsigned overflow
        let r_dist = (self.r as i64) - (other.r as i64);
        let g_dist = (self.g as i64) - (other.g as i64);
        let b_dist = (self.b as i64) - (other.b as i64);
        (r_dist*r_dist + g_dist*g_dist + b_dist*b_dist) as u32
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
}
