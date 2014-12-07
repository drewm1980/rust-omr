#![allow(dead_code)]

//extern crate omr;

use std::os;
use std::io::fs::PathExtensions;

//mod lib;
//mod loaders;
//use loaders::load;
//mod pgm;

fn main() {
    let args = os::args();
    println!("Entering main...");
    let path_to_load = match args.len() {
        0|1 => {println!("Too few arguments passed!");
            panic!("Usage: rust-omr PATH");},
            2 => {println!("The first argument is {}", args[1]);
                &args[1]},
                _ => {println!("Too many arguments passed!");
                    panic!("Usage: rust-omr PATH");},
    };

    let loadpath = Path::new(path_to_load);
    if ! (loadpath.exists()) {panic!("The file {} does not exist!",path_to_load)}

    //loaders::load(&loadpath);
}
