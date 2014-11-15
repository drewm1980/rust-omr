#![crate_name = "rust-omr"]
#![crate_type = "rlib"]

//use std::mem;
//use std::io;
//use std::io::File;
//use std::ptr;
//use std::slice;
use std::os;

use std::io::fs::PathExtensions;

mod loaders;
//use loaders::load;

#[allow(dead_code)]
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

    loaders::load(&loadpath);
}
