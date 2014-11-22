/// Functions for loading and saving pgm images.
///
/// For documentation of the PGM image format see:
///  http://netpbm.sourceforge.net/doc/pgm.html

//use librust_omr::Image;
//mod lib;
use image::Image;
use std::u32;

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

fn isdigit(c:u8) -> bool {
    match c {
        30|31|32|33|34|35|36|37|38|39 => true,
        _ => false
    }
}

/// Load an image that is already known to be a pgm file
pub fn load(path: &Path) -> Image {

    use std::io::File;
    let contents = match File::open(path).read_to_end() {
        Ok(c) => c,
        Err(e) => panic!("Could not read to the end of the file!: {}",e),
    };

    parse(contents.as_slice())
}

/// Parse an in-memory pgm file
pub fn parse(data:&[u8]) -> Image {

    // Scan a the magic word "P5"
    let iter = data.iter();
    let mut c:u8;

    // Get the next c if there is one, or panic.
    let nextc = || { c = *(iter.next().unwrap()); };

    nextc();
    assert!(c == 'P' as u8);
    nextc();
    assert!(c == '5' as u8);

    let eat_at_least_some_whitespace = || {
        nextc();
        assert!(isspace(c));
        while isspace(c) {nextc()}; 
    };

    eat_at_least_some_whitespace();

    let read_a_digit = || {
        assert!(isdigit(c));
        let s:String;
        while isdigit(c) {
            s.push(c as char);
            nextc();
        }
        match from_str::<u32>(s.as_slice()) {
            Some(i) => i,
            None => panic!("Could not convert string to int.  Corrupted pgm file?"),
        }
    };

    let width = read_a_digit();
    
    eat_at_least_some_whitespace();

    let height = read_a_digit();

    eat_at_least_some_whitespace();
    
    let maxval = read_a_digit();
    assert_eq!(maxval,255);

    // There should be exactly one more whitespace character according to the spec.
    nextc();
    assert!(isspace(c));

    let mut pixels:Vec<u8> = Vec::with_capacity((width*height) as uint);

    for p in iter {
        pixels.push(*p);
    }
    assert!(pixels.len()==(width*height) as uint, "PGM image has wrong number of pixels according to the header!");
    
    // Get the data into our own data structure.
    Image {
        width: width,
        height: height,
        pixels: pixels.as_slice(),
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

