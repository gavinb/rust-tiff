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

use std::io::{IoResult, IoError, IoErrorKind, Reader, Seek, SeekSet};
use std::io::fs::File;

use {ByteOrder, HeaderMagic, TIFFHeader, IFD, IFDEntry, decode_tag, decode_tag_type, SeekableReader};

pub struct TIFFReader;

impl TIFFReader {

    pub fn load(&self, filename: &str) -> IoResult<u32> {

        let filepath = Path::new(filename);
        let mut reader = File::open(&filepath);

        self.read(&mut reader)
    }

    pub fn read(&self, reader: &mut SeekableReader) -> IoResult<u32> {

        // @todo Ensure file is >= min size

        // Read and validate ByteOrder

        let byte_order_field = try!(reader.read_le_u16());
        let byte_order: ByteOrder;

        if byte_order_field == ByteOrder::LittleEndian as u16 {
            byte_order = ByteOrder::LittleEndian;
        } else if byte_order_field == ByteOrder::BigEndian as u16 {
            byte_order = ByteOrder::BigEndian;
        } else {
            return Err(IoError {
                kind: IoErrorKind::OtherIoError,
                detail: Some(String::from_str("Invalid byte order in header")),
                desc: "",
            });
        }

        // Read and validate HeaderMagic

        let magic_field = try!(reader.read_le_u16());
        let magic: HeaderMagic;

        if magic_field == HeaderMagic::LittleEndian as u16 {
            magic = HeaderMagic::LittleEndian;
        }
        else if magic_field == HeaderMagic::BigEndian as u16 {
            magic = HeaderMagic::BigEndian;
        } else {
            return Err(IoError {
                kind: IoErrorKind::OtherIoError,
                detail: Some(String::from_str("Invalid magic number in header")),
                desc: "",
            });
        }

        // Read offset to first IFD

        let ifd_offset_field = try!(reader.read_le_u32());

        // Assemble validated header

        let header = TIFFHeader {
            byte_order: byte_order,
            magic: magic,
            ifd_offset: ifd_offset_field,
        };

        try!(reader.seek(ifd_offset_field as i64, SeekSet));
        println!("IFD offset: {}", ifd_offset_field);

        try!(self.read_IFD(reader));

        Ok(42)
    }

    #[allow(non_snake_case)]
    fn read_IFD(&self, reader: &mut SeekableReader) -> IoResult<Box<IFD>> {

        let entry_count = try!(reader.read_le_u16());

        let mut ifd = box IFD { count: entry_count, entries: Vec::with_capacity(entry_count as uint) };

        println!("IFD entry count: {}", entry_count);

        for entry_number in range(0, entry_count as uint) {
            ifd.entries.push(*self.read_tag(entry_number, reader).unwrap());
        }

        Ok(ifd)
    }

    fn read_tag(&self, entry_number: uint, reader: &mut SeekableReader) -> IoResult<Box<IFDEntry>> {

        let tag_value = try!(reader.read_le_u16());
        let typ_value = try!(reader.read_le_u16());
        let count_value = try!(reader.read_le_u32());
        let value_offset_value = try!(reader.read_le_u32());

        let tag = decode_tag(tag_value).expect(format!("Invalid tag {:x}", tag_value).as_slice());
        let typ = decode_tag_type(typ_value).expect(format!("Invalid tag type {:x}", typ_value).as_slice());

        let e0 = box IFDEntry {
            tag: tag,
            typ: typ,
            count: count_value,
            value_offset: value_offset_value,
        };

        println!("IFD[{}] {} {} {:x} {}", entry_number, e0.tag, e0.typ, e0.count, e0.value_offset);
        Ok(e0)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_tiff() {
    }
}
