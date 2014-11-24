
extern crate "rust-tiff" as tiff;

#[test]
fn test_load() {
    let tiff_reader = tiff::TIFFReader;
    match tiff_reader.load("../resources/text_w320_h200_b8_cn_prgb.tiff") {
        Ok(x) => println!("Read tiff {}", x),
        Err(e) => println!("File I/O Error: {}", e),
    }
}
