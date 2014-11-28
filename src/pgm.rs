/// Functions for loading and saving pgm images.
///
/// For documentation of the PGM image format see:
///  http://netpbm.sourceforge.net/doc/pgm.html

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
    let contents:Vec<u8> = match File::open(path).read_to_end() {
        Ok(c) => c,
        Err(e) => panic!("Could not read to the end of the file!: {}",e),
    };

    parse(contents)
}

fn parse_int(s:&[u8]) -> int {
    let mut n:Vec<u8> = vec![];
    let i:int = 0;
    for i in range(0,s.len()){
        match s[0] {
            d if isdigit(d) => n.push(d),
            _ => break,
        }
    }
    0
}

/// Parse an in-memory pgm file
pub fn parse(data:Vec<u8>) -> Image {
    parse_int(data.as_slice());

    //// Scan a the magic word "P5"
    //let mut iter = data.iter();
    //let mut c:u8;

    //// Get the next c if there is one, or panic.
    ////let next_or_panic = || { *(iter.next().unwrap()) };
    //fn next_or_panic (
        //iter: Iterator<u8>, 
        //c: u8,
        //) -> () { 
        //*c = *(iter.next().unwrap());
        //(iter,c)
    //};

    //fn eat_at_least_some_whitespace (
        //iter: &mut Iterator<u8>, 
        //c: &mut u8,
        //) {
        //*c = next_or_panic(*iter,c);
        //assert!(isspace(*c));
        //while isspace(*c) {*c = next_or_panic(*iter)}; 
    //};

    //fn read_a_digit = () {
        //assert!(isdigit(c));
        //let s:String;
        //while isdigit(c) {
            //s.push(c as char);
            //c = next_or_panic(iter);
        //}
        //match from_str::<u32>(s.as_slice()) {
            //Some(i) => i,
            //None => panic!("Could not convert string to int.  Corrupted pgm file?"),
        //}
    //};


    //(iter,c) = next_or_panic(&iter,&c);

    //assert!(c == 'P' as u8);
    //c = next_or_panic(&mut iter,&mut c);
    //assert!(c == '5' as u8);

    ////eat_at_least_some_whitespace();

    ////let width = read_a_digit();
    //let width:u8 = 10;
    
    ////eat_at_least_some_whitespace();

    ////let height = read_a_digit();
    //let height:u8 = 10;

    ////eat_at_least_some_whitespace();
    
    ////let maxval = read_a_digit();
    //let maxval:u8 = 255;
    ////assert_eq!(maxval,255);

    //// There should be exactly one more whitespace character according to the spec.
    //c = next_or_panic(&iter,&c);
    //assert!(isspace(c));

    //let mut pixels:Vec<u8> = Vec::with_capacity((width*height) as uint);

    //for p in iter {
        //pixels.push(*p);
    //}
    //assert!(pixels.len()==(width*height) as uint, "PGM image has wrong number of pixels according to the header!");
    
    // Get the data into our own data structure.
    Image {
        width: 10,
        height: 10,
        pixels: vec![1,2,3],
    }
    //Image {
        //width: width,
        //height: height,
        //pixels: pixels,
    //}
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

