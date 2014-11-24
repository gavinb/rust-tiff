
extern crate "rust-tiff" as tiff;

use std::os;

fn print_header(x: &tiff::TIFFHeader) {
    println!("byte_order: {}", x.byte_order);
    println!("magic:      {}", x.magic);
    println!("ifd_offset: {}", x.ifd_offset);
}

fn main() {
    let tiff_reader = tiff::TIFFReader;
    match tiff_reader.load(os::args()[1].as_slice()) {
        Ok(h) => {
            println!("Read tiff {}", h);
            print_header(&*h);
        },
        Err(e) => {
            println!("File I/O Error: {}", e);
        }
    }
}
