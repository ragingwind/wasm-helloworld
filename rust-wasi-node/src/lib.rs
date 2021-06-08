use std::io::prelude::*;
use std::fs;

#[no_mangle]
pub extern "C" fn print_hello() {
    println!("Hello, world!");
    let mut file = fs::File::create("host/helloworld.txt").unwrap();

  // Write the text to the file we created
  write!(file, "Hello world!\n").unwrap();
}
