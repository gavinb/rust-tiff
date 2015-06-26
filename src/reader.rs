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

use std::io::{Result, Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;
use std::fs::File;

use {ByteOrder, HeaderMagic, TIFFHeader, IFD, IFDEntry, decode_tag, decode_tag_type, SeekableReader};

pub struct TIFFReader;

impl TIFFReader {

    pub fn load(&self, filename: &str) -> Result<Box<TIFFHeader>> {

        let filepath = Path::new(filename);
        let mut reader = File::open(&filepath).unwrap();

        self.read(&mut reader)
    }

    pub fn read(&self, reader: &mut SeekableReader) -> Result<Box<TIFFHeader>> {

        // @todo Ensure file is >= min size

        // Read and validate ByteOrder

        let byte_order_field = try!(reader.read_le_u16());
        let byte_order: ByteOrder;

        if byte_order_field == ByteOrder::LittleEndian as u16 {
            byte_order = ByteOrder::LittleEndian;
        } else if byte_order_field == ByteOrder::BigEndian as u16 {
            byte_order = ByteOrder::BigEndian;
        } else {
            return Err(Error::new(ErrorKind::Other,
                                  String::from_str("Invalid byte order in header")));
        }
        println!("byte_order {:?}", byte_order);

        // Read and validate HeaderMagic

        let magic_field = match byte_order {
            ByteOrder::LittleEndian => try!(reader.read_le_u16()),
            ByteOrder::BigEndian => try!(reader.read_be_u16()),
        };

        let magic: HeaderMagic;

        if magic_field == HeaderMagic::LittleEndian as u16 {
            magic = HeaderMagic::LittleEndian;
        }
        else if magic_field == HeaderMagic::BigEndian as u16 {
            magic = HeaderMagic::BigEndian;
        } else {
            return Err(Error::new(ErrorKind::Other, String::from_str("Invalid magic number in header")));
        }

        // Read offset to first IFD

        let ifd_offset_field = match byte_order {
            ByteOrder::LittleEndian => try!(reader.read_le_u32()),
            ByteOrder::BigEndian => try!(reader.read_be_u32()),
        };

        // Assemble validated header

        let header = Box::new(TIFFHeader {
            byte_order: byte_order,
            magic: magic,
            ifd_offset: ifd_offset_field,
        });

        try!(reader.seek(ifd_offset_field as i64, SeekFrom::Start));
        println!("IFD offset: {:?}", ifd_offset_field);

        try!(self.read_IFD(reader));

        Ok(header)
    }

    #[allow(non_snake_case)]
    fn read_IFD(&self, reader: &mut SeekableReader) -> Result<Box<IFD>> {

        let entry_count = try!(reader.read_be_u16());

        println!("IFD entry count: {}", entry_count);

        let mut ifd = Box::new(IFD { count: entry_count, entries: Vec::with_capacity(entry_count as usize) });

        println!("IFD entry count: {}", entry_count);

        for entry_number in 0..entry_count as usize {
            ifd.entries.push(*self.read_tag(entry_number, reader).unwrap());
        }

        Ok(ifd)
    }

    fn read_tag(&self, entry_number: usize, reader: &mut SeekableReader) -> Result<Box<IFDEntry>> {

        let tag_value = try!(reader.read_le_u16());
        let typ_value = try!(reader.read_le_u16());
        let count_value = try!(reader.read_le_u32());
        let value_offset_value = try!(reader.read_le_u32());

        let tag = decode_tag(tag_value).expect(format!("Invalid tag {:x}", tag_value).as_str());
        let typ = decode_tag_type(typ_value).expect(format!("Invalid tag type {:x}", typ_value).as_str());

        let e0 = Box::new(IFDEntry {
            tag: tag,
            typ: typ,
            count: count_value,
            value_offset: value_offset_value,
        });

        println!("IFD[{:?}] {:?} {:?} {:x} {}", entry_number, e0.tag, e0.typ, e0.count, e0.value_offset);
        Ok(e0)
    }
}
