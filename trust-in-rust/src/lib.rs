use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn handle(_req : String) -> String {
    let mut f = File::open("function/src/index.html").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    contents
}