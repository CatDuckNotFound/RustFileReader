use std::io;
use std::fs::{File,OpenOptions};
use std::io::prelude::*;
#[allow(unused)]
fn main() {


//let mut new_f = File::create("src/created.txt").expect("file sec err");
    //new_f.write_all(b"hellow texttt").expect("erreoes");



let mut new_file = OpenOptions::new().append(true).open("src/created.txt").expect("failed to open");
new_file.write_all(b"wowww fucked upp \n hello shitt").expect("failed to write");

let mut readable = File::open("src/created.txt").expect("couln't read msg");
    let mut content = String::new();
    readable.read_to_string(&mut content).unwrap();
    println!("{}",content);

}


