#![allow(unused_imports)]

/// Functions for loading and saving pgm images.
///
/// For documentation of the PGM image format see:
///  http://netpbm.sourceforge.net/doc/pgm.html

use image::Image;

/// Test if a char is (UNIX) whitespace.
fn is_space(c:u8) -> bool {
    match c as char {'\x20'// space (SPC)
            |'\x09'	// horizontal tab (TAB)
            |'\x0a'	// newline (LF)
            |'\x0b'	// vertical tab (VT)
            |'\x0c'	// feed (FF)
            |'\x0d'	// carriage return (CR)
            => true,
            _=> false
    }
}

/// Test if a char is an ASCII digit
fn is_digit(c:u8) -> bool {
    //match c {
        //30|31|32|33|34|35|36|37|38|39 => true,
        //_ => false
    //}
    c>=0x29 && c<0x40
    //match c {
        //30|31|32|33|34|35|36|37|38|39 => true,
        //_ => false
    //}
}

/// Parse an integer from the front of an ascii string,
/// and return it along with the remainder of the string
fn parse_int(s:&[u8]) -> (u32, &[u8]) {
    use std::str;
    assert!(s.len()>0);
    let mut newslice = s; // bytecopy of the fat pointer?
    let mut n:Vec<u8> = vec![];

    // Pull the leading digits into a separate array
    while newslice.len()>0 && is_digit(newslice[0])
    {
        n.push(newslice[0]);
        newslice = newslice.slice_from(1);
        //newslice = newslice[1..];
    }

    match from_str::<u32>(str::from_utf8(n.as_slice()).unwrap()) {
        Some(i) => (i,newslice),
        None => panic!("Could not convert string to int.  Corrupted pgm file?"),
    }
}

///// Remove all of the leading whitespace from a ascii string
//fn remove_leading_whitespace(s:&[u8]) -> &[u8] {
    //while s.len()>0 && is_space(s[0])
    //{
        //let s = s.slice(1,s.len()-1);
    //}
    //s
//}

/// Load an image that is already known to be a pgm file
pub fn load(path: &Path) -> Image {

    use std::io::File;
    let contents:Vec<u8> = match File::open(path).read_to_end() {
        Ok(c) => c,
        Err(e) => panic!("Could not read to the end of the file!: {}",e),
    };

    parse(contents.as_slice())
}

/// Parse an in-memory pgm file
#[allow(unreachable_code)]
pub fn parse(s:&[u8]) -> Image {
    
    // Scan the magic word "P5"
    assert!(s.len()>2);
    assert!(s[0] == 'P' as u8);
    assert!(s[1] == '5' as u8);
    let s = s.slice(2,s.len());

/*
    let s = remove_leading_whitespace(s);

    panic!("HERE!!");

    let (width,height,maxval):(int,int,int);
    let (width,s) = parse_int(s.as_slice());

    let s = remove_leading_whitespace(s);

    let (height,s) = parse_int(s.as_slice());

    let s = remove_leading_whitespace(s);
    
    let (maxval,s) = parse_int(s.as_slice());
    assert_eq!(maxval,255);

    // Exactly one character of whitespace is obligatory
    assert!(is_space(s[0]));
    let s = s.slice(1,s.len()); 
 
    println!("Parsed out width={}, hight={}, maxval={}",width,height,maxval);
    
    // What remains in our slice is the slice we care about
    let mut pixels:Vec<u8> = Vec::with_capacity((width*height) as uint);

    for &p in s.iter() {
        pixels.push(p);
    }
    assert!(pixels.len()==(width*height) as uint, "PGM image has wrong number of pixels according to the header!");
    */
    
    //Image {width::2,height:2,pixels:b"1234"}
    let width = 2;
    let height = 2;
    let pixels:Vec<u8> = vec![1,2,3,4];

    Image {
        width: width,
        height: height,
        pixels: pixels,
    }
}

#[cfg(test)]
mod test {
	extern crate test;
    use image::Image;

#[test]
    fn test_is_space() {
        let s:&[u8] = b" 2";
        assert!(super::is_space(s[0]));
        assert!(!super::is_space(s[1]));
    }

#[test]
    fn test_is_digit() {
        let s:&[u8] = b"0123456789";
        for &c in s.iter() {
            assert!(super::is_digit(c));
        }

        let s2:&[u8] = b"abcdefghijklmnopqsttuvwxyz";
        for &c in s2.iter() {
            assert!(!super::is_digit(c));
        }
    }

#[test]
    fn test_parse_int() {
        let s:&[u8] = b"012345";
        assert!(s.len()==6);
        let (i,newslice) = super::parse_int(s);
        assert!(i==12345);
        println!("length of returned slice: {}",newslice.len());
        assert!(newslice.len()==0);
    }

#[test]
    fn test_parse_int_with_trailing() {
        let s:&[u8] = b"102345 abcd";
        assert!(s.len()==11);
        let (i,newslice) = super::parse_int(s);
        assert!(i==102345);
        assert!(newslice.len()==5);
        assert!(newslice[0]==b' ');
    }


//#[test]
    //use super::parse;
    //fn test_parse_trivial_inmemory() {
        //let s:&[u8] = b"P5 2 2 255 12345678";
        //let i:Image = parse(s);
    //}

//#[test]
	//fn test_load_pgm_from_file() {
		//let loadfile = "test_images/rust_favicon.pgm";
		//let loadpath = &Path::new(loadfile);
		//super::load(loadpath);
	//}

}

