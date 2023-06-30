use obstinate;
use std::{fs::File, str::from_utf8};

pub fn main() {
    let file = File::open("examples/files/hello_world.txt").unwrap();
    let mmaped = unsafe { obstinate::Mmap::map(&file) }.unwrap();
    print!("content: {}.", from_utf8(&mmaped[..]).unwrap());
}
