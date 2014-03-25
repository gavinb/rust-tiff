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

#[license = "MIT"];

use tiff;

struct TIFFWriter;

impl TIFFWriter {

    fn save(&self, filename: &str) -> IoResult<u32> {
    }
}
