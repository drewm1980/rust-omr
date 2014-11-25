//#![crate_name = "rust_omr"]
//#![crate_type = "rlib"]
//! This module contains the API for a toy Optical Music Recognition System.


#![allow(dead_code)]
#![allow(unused_variables)]

use std::default::Default;
use std::fmt;

use image::Image;
mod image;
mod loaders;
//mod pgm;
//use image::Image;
//use 


//#[deriving(Default)] // Broken in rust nightly
//#[deriving(Show)] // Also Broken in rust nightly
struct Histogram {
    sum: u32,
    bins: [u32, ..256],
}

impl Default for Histogram {
    #[inline]
    fn default() -> Histogram { 
        Histogram{
            sum:0,
            bins:[0,..256],
        }
    }
}

impl fmt::Show for Histogram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s:String;
        let mut s = Vec::new();
        for p in self.bins.iter() {
            write!(&mut s, "{}, ", p);
        }
        write!(f, "Histogram({})", s)
    }
}

/// Single pass algorithm to compute the histogram of 
/// the pixel colors in a grayscale 8 bit image,
/// or other flat dense array of u8.
fn histogram(im:image::Image) -> Histogram {
    let numpixels:u64 = (im.width as u64) * (im.height as u64);
    let needed_bin_capacity:u64 = 255*numpixels;
    //if needed_bin_capacity > 2**32-1
    //{
    let mut h:Histogram = Default::default();
    for p in im.pixels.iter() {
        h.bins[*p as uint] += 1; // Probably has bounds check!?!
        h.sum += 1;
    }
    //}
    h
}


#[cfg(test)]
mod test {
	extern crate test;

    use std::default::Default;
    use super::{Histogram};

#[test]
    fn test_histogram_creation() {
        let mut h:Histogram = Default::default();

    }

#[test]
    fn test_histogram_print() {
        let mut h:Histogram = Default::default();
        println!("Contents of the histogram: {}",h);
    }

//#[test]
    //fn test_histogram_sum() {
        //let mut h1:Histogram = Default::default();
        //let mut h2:Histogram = Default::default();
        ////sum(h1,h2)
    //}
}

