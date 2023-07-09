use obstacle;
use std::{fs::File, str::from_utf8};

pub fn main() {
    let file = File::open("examples/files/hello_world.txt").unwrap();
    let mmaped = unsafe { obstacle::Mmap::map(&file) }.unwrap();
    print!("content: {}.", from_utf8(&mmaped[..]).unwrap());
}
