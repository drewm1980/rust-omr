extern crate png;
use self::png::{load_png,Image};

pub fn load(path: &Path) -> png::Image {
		println!("Loading file {}...",path.display());

		let img = match load_png(path){
			Ok(img) => img,
			Err(e) => panic!("Could not open file at path: {}; load_path returned {}",path.display(),e)
		};
		println!("Image width is {}", img.width);
		img
}

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

//#[test]
	//fn test_store() {
		//let mut img = Image {
//width: 10,
	       //height: 10,
	       //pixels: RGB8(Vec::from_elem(10 * 10 * 3, 100u8)),
		//};
		//let res = store_png(&mut img, &Path::new("test/store.png"));
		//assert!(res.is_ok());
	//}
//#[test]
	//fn test_load() 
	//{
		//load_rgba8("test/servo-screenshot.png", 831, 624);
		//load_rgba8("test/store.png", 10, 10);
	//}
}

