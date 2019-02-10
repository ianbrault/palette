/*
 * pixel.rs
 * pixel data structure implementation
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use crate::kmeans::GenericVector;

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

    pub fn as_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.b, self.g)
    }

    pub fn as_rgba(&self) -> image::Rgba<u8> {
        image::Rgba([self.r, self.g, self.b, 1])
    }
}

impl GenericVector for Pixel {
    fn average<'a, I>(vectors: I) -> Pixel
        where Pixel: 'a, I: Iterator<Item=&'a Pixel>
    {
        let mut n = 0;
        let mut sum: (u64, u64, u64) = (0, 0, 0);
        for v in vectors {
            n += 1;
            sum.0 += u64::from(v.r);
            sum.1 += u64::from(v.g);
            sum.2 += u64::from(v.b);
        }

        let r = sum.0 / n;
        let g = sum.1 / n;
        let b = sum.2 / n;

        Pixel::new(r as u8, g as u8, b as u8)
    }

    fn distance(&self, other: &Pixel) -> u32 {
        // cast up to i64 to avoid unsigned overflow
        let r_dist = i64::from(self.r) - i64::from(other.r);
        let g_dist = i64::from(self.g) - i64::from(other.g);
        let b_dist = i64::from(self.b) - i64::from(other.b);
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
