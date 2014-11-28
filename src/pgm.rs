/// Functions for loading and saving pgm images.
///
/// For documentation of the PGM image format see:
///  http://netpbm.sourceforge.net/doc/pgm.html

use image::Image;
use std::u32;

/// Test if a char is (UNIX) whitespace.
fn isspace(c:u8) -> bool {
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
    match c {
        30|31|32|33|34|35|36|37|38|39 => true,
        _ => false
    }
}

///// Parse a single digit from the beginning of an ascii string,
///// and return the rest of the string.
//fn parse_digit(s:&[u8]) -> (u8, &[u8]) {
    //assert!(s.len()>0);
    //if is_digit(s[0]) 
    //{
        //(s[0],s.slice(1,s.len()-1))
    //} else {
        //panic!("Couldn't parse a digit from the front of the string!")
    //}
//}

/// Parse an integer from the front of an ascii string,
/// and return it along with the remainder of the string
fn parse_int(s:&[u8]) -> (u32, &[u8]) {
    assert!(s.len()>0);
    let mut n:Vec<u8> = vec![];
    while (s.len()>0 && is_digit(s[0]))
    {
        n.push(s[0]);
        s = s.slice(1,s.len()-1);
    }

    let i:int = 0;
    for i in range(0,s.len()){
    }

    match from_str::<u32>(s) {
        Some(i) => (i,s),
        None => panic!("Could not convert string to int.  Corrupted pgm file?"),
    }
}

/// Remove all of the leading whitespace from a ascii string
fn remove_leading_whitespace(s:&[u8]) -> &[u8] {
    while(s.len()>0 && isspace(s[0]))
    {
        s = s.slice(1,s.len()-1);
    }
    s
}

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
pub fn parse(s:&[u8]) -> Image {
    
    // Scan the magic word "P5"
    assert!(s.len()>2);
    assert!(s[0] == 'P' as u8);
    assert!(s[1] == '5' as u8);
    s = s.slice(2,s.len());

    s = remove_leading_whitespace(s);

    let (width:int,s) = parse_int(data.as_slice());

    s = remove_leading_whitespace(s);

    let (height:int,s) = parse_int(data.as_slice());

    s = remove_leading_whitespace(s);
    
    let (maxval:int,s) = parse_int(data.as_slice());
    assert_eq!(maxval,255);

    // One character of whitespace is obligatory
    assert!(isspace(s[0]));
    s = s.slice(1,s.len()); 
    
    // What remains in our slice is the slice we care about
    let mut pixels:Vec<u8> = Vec::with_capacity((width*height) as uint);

    for &p in s.iter() {
        pixels.push(p);
    }
    assert!(pixels.len()==(width*height) as uint, "PGM image has wrong number of pixels according to the header!");
    
    // Get the data into our own data structure.
    //Image {
        //width: 10,
        //height: 10,
        //pixels: vec![1,2,3],
    //}
    Image {
        width: width,
        height: height,
        pixels: pixels,
    }
}

#[cfg(test)]
mod test {
	extern crate test;

#[test]
	fn test_load_pgm_from_file() {
		let loadfile = "test_images/rust_favicon.pgm";
		let loadpath = &Path::new(loadfile);
		super::load(loadpath);
	}

}

