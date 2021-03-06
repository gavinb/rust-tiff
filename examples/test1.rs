//============================================================================
//
//  A Tagged Image File Format (TIFF) Library for Rust
//
//  Based on the TIFF 6.0 specification:
//
//      https://partners.adobe.com/public/developer/en/tiff/TIFF6.pdf
//
//  Copyright (c) 2014 by Gavin Baker <gavinb@antonym.org>
//  Published under the MIT License
//
//============================================================================

extern crate "rust-tiff" as tiff;

use std::os;

use tiff::reader::TIFFReader;

//----------------------------------------------------------------------------
//static HeaderOffset

fn main() {
    let tiff_reader = TIFFReader;
    match tiff_reader.load(os::args()[1].as_slice()) {
        Ok(x) => println!("Read tiff {}", x),
        Err(e) => println!("File I/O Error: {}", e),
    }
}
