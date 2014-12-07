use pgm;
use image::{Image};
use std::str::{from_utf8};

/// Load an image file using ImageMagick in a separate process,
/// with automatic fallback (and fast case) to a native pgm file loader.
pub fn load(path: &Path) -> Image {
    match path.extension() {
        Some(s) if from_utf8(s).unwrap()=="pgm" => pgm::load(path),
        _ => load_using_magick(path),
    }
}

// Load a file by first using imageMagick to convert it to a .pgm file.
fn load_using_magick(path: &Path) -> Image {
    use std::io::Command;
    use std::io::process::ProcessExit::{ExitStatus,ExitSignal};

    let output = Command::new("convert")
        .arg(path)
        .arg("pgm:-") // Dark imagemagick magick to force output of specific image type to stdout
        .output().ok().unwrap();
    
    let data: Vec<u8> = match output.status {
        ExitStatus(i) if i==0 => output.output,
        ExitStatus(i) => panic!("Image Magick reported an error: {}",String::from_utf8_lossy(output.error.as_slice())),
        ExitSignal(i) => panic!("Convert subprocess threw a signal {}",i),
    };

    pgm::parse(data.as_slice())
}

#[cfg(test)]
mod test {
	extern crate test;

#[test]
	fn test_load_png_from_file() {
		let loadfile = "test_images/rust_favicon.png";
		let loadpath = Path::new(loadfile);
		super::load(&loadpath);
	}

#[test]
	fn test_load_jpg_from_file() {
		let loadfile = "test_images/rust_favicon.jpg";
		let loadpath = Path::new(loadfile);
		super::load(&loadpath);
	}

#[test]
	fn test_load_gif_from_file() {
		let loadfile = "test_images/rust_favicon.gif";
		let loadpath = Path::new(loadfile);
		super::load(&loadpath);
	}

#[test]
	fn test_load_non_toy_image() {
		let loadfile = "sheet_music/La_yumba1.gif";
		let loadpath = Path::new(loadfile);
		super::load(&loadpath);
	}

}

