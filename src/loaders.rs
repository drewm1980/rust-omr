//extern crate png;
//extern crate librust_omr;

use image::{Image};
use pgm;

pub fn load(path: &Path) -> Image {
    match path.extension {
        //Some(str) if str=="png" => load_using_png(path),
        Some(str) if str=="pgm" => pgm::load(path),
       None => load_using_magick(path),
    }
}

// Load a file by first using imageMagick to convert it to a .pgm file.
fn load_using_magick(path: &Path) -> Image {
    use std::io::Command;

    let output = match Command::new("convert")
        .arg("-format pgm")
        .arg("-depth 8")
        .arg(path)
        .arg("-")
        .output() {
        Ok(output) => output,
        Err(e) => panic!("Unable to run ImageMagick's convert tool in a separate process! convert returned: {}", e),
    };

    //println!("status: {}", output.status);
    //println!("stdout: {}", String::from_utf8_lossy(output.output.as_slice()));
    //println!("stderr: {}", String::from_utf8_lossy(output.error.as_slice()));

    pgm::parse(data)

}

//fn load_using_png(path: &Path) -> Image {
    //use pgm;

    //let img = match png::load_png(path){
        //Ok(img) => img,
        //Err(e) => panic!("Could not open file at path: {}; load_path returned {}",path.display(),e),
    //};
    //println!("Image width is {}", img.width);

    //// Get the data into our own data structure.
    //Image {
        //width: img.width,
        //height: img.height,
        //pixels: match img.pixels {
            //png::K8(buf) => buf.as_slice(),
            //_ => panic!("Image pixel type is unsupported!"),
        //},
    //}
//}

#[cfg(test)]
mod test {
	extern crate test;
	//extern crate png;
	//use self::png::{load_png,Image};

#[test]
	fn test_easyload() {
		let loadfile = "sheet_music/La_yumba1.png";
		let loadpath = &Path::new(loadfile);
		super::load(loadpath);
	}

#[test]
	fn test_load_png_from_file() {
		let loadfile = "test_images/rust_favicon.png";
		let loadpath = &Path::new(loadfile);
		super::load(loadpath);
	}

#[test]
	fn test_load_jpg_from_file() {
		let loadfile = "test_images/rust_favicon.jpg";
		let loadpath = &Path::new(loadfile);
		super::load(loadpath);
	}

#[test]
	fn test_load_gif_from_file() {
		let loadfile = "test_images/rust_favicon.gif";
		let loadpath = &Path::new(loadfile);
		super::load(loadpath);
	}
}

