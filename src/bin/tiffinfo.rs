
extern crate rust_tiff as tiff;

use std::env;

fn print_header(x: &tiff::TIFFHeader) {
    println!("byte_order: {:?}", x.byte_order);
    println!("magic:      {:?}", x.magic);
    println!("ifd_offset: {}", x.ifd_offset);
}

fn main() {
    let tiff_reader = tiff::TIFFReader;
    let file_name = env::args().nth(1).unwrap();
    println!("tiffinfo: {}", file_name);
    match tiff_reader.load(&file_name) {
        Ok(h) => {
            println!("Read tiff {:?}", h);
            print_header(&*h);
        },
        Err(e) => {
            println!("File I/O Error: {}", e);
        }
    }

    // Test image profile validation
    let all_types = [
        tiff::ImageType::Bilevel,
        tiff::ImageType::Grayscale,
        tiff::ImageType::PaletteColour,
        tiff::ImageType::RGB,
        tiff::ImageType::YCbCr];
    println!("Image validity test:");
    for typ in all_types.iter() {

        let ans = tiff::validate_required_tags_for(typ);
        match ans {
            Some(missing_tags) => {
                println!("  {:?}: invalid, missing:", typ);
                for x in &missing_tags {
                    println!("    - {:?}", x);
                }
            },
            None => {
                println!("  {:?}: Valid ./", typ);
            },
        }
    }

}
