/// Functions for loading and saving pgm images.
///
/// For documentation of the PGM image format see:
///  http://netpbm.sourceforge.net/doc/pgm.html

//use librust_omr::Image;
//mod lib;
//use super::Image;

fn isspace(c:u8) -> bool {
    match c {'\x20'// space (SPC)
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
    let contents = File::open(path).read_to_end();

    parse(contents)
}

pub fn parse(data) -> Image {

    // Scan a the magic word "P5"
    let iter = contents.iter();
    assert!(iter.next()=='P');
    assert!(iter.next()=='5');

    let eat_at_least_some_whitespace = || {
        assert!(isspace(iter.next()));
        let mut c = iter.next();
        while isspace(c) { c = iter.next() }; 
    }

    eat_at_least_some_whitespace();

    // Scan a digit representing the width
    let read_a_digit = || {
        assert!(isdigit(c));
        let s:String;
        while isdigit(c) {
            s.push(c);
            c = iter.next();
        }
        s
    }

    let width = read_a_digit();
    
    eat_at_least_some_whitespace();

    let height = read_a_digit();

    eat_at_least_some_whitespace();
    
    let maxval = read_a_digit();
    asserteq!(maxval,255);

    // There should be exactly one more whitespace character according to the spec.
    assert!(isspace(iter.next()));

    let pixels = vec![_,..width*height];

    for p in pixels {
        pixels.push(
            match iter.next(){
                Some(byte) => byte;
                None => panic!("PGM image had too few pixels!")
            });
    }
    
    // Get the data into our own data structure.
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
		let loadfile = "sheet_music/La_yumba1.png";
		let loadpath = &Path::new(loadfile);
		super::load(loadpath);
	}
}
