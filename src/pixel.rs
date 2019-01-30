/*
 * pixel.rs
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    // store squared channels for 2-norm
    r_sq: u8,
    g_sq: u8,
    b_sq: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {
            r, g, b,
            r_sq: r * r,
            g_sq: g * g,
            b_sq: b * b,
        }
    }
}
