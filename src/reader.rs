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

use byteorder::{ReadBytesExt, WriteBytesExt, ByteOrder, BigEndian, LittleEndian};

use {TIFFByteOrder, HeaderMagic, TIFFHeader, IFD, IFDEntry, decode_tag, decode_tag_type, SeekableReader};

pub struct TIFFReader;

impl TIFFReader {

    pub fn load(&self, filename: &str) -> Result<Box<TIFFHeader>> {

        let filepath = Path::new(filename);
        let mut reader = File::open(&filepath).unwrap();

        self.read(&mut reader)
    }

    pub fn read(&self, reader: &mut SeekableReader) -> Result<Box<TIFFHeader>> {

        let byte_order = self.read_byte_order(reader);

        let magic = match byte_order {
            Ok(TIFFByteOrder::LittleEndian) => self.read_magic(reader),
            Ok(TIFFByteOrder::BigEndian) => self.read_magic(reader),
            Err(e) => Err(e)
        };

        self.read_::<BigEndian>(reader)
    }

    pub fn read_byte_order(&self, reader: &mut SeekableReader) -> Result<TIFFByteOrder> {

        // Bytes 0-1: "II" or "MM"
        // Read and validate ByteOrder

        let byte_order_field = try!(reader.read_u16::<LittleEndian>());
        let byte_order: TIFFByteOrder;

        if byte_order_field == TIFFByteOrder::LittleEndian as u16 {
            byte_order = TIFFByteOrder::LittleEndian;
        } else if byte_order_field == TIFFByteOrder::BigEndian as u16 {
            byte_order = TIFFByteOrder::BigEndian;
        } else {
            return Err(Error::new(ErrorKind::Other,
                                  format!("Invalid byte order in header: {:04x}", byte_order_field)));
        }
        println!("byte_order {:?}", byte_order);

        Ok(byte_order)
    }

    pub fn read_magic(&self, reader: &mut SeekableReader) -> Result<HeaderMagic> {

        // Bytes 2-3: 0042
        // Read and validate HeaderMagic

        let magic_field = try!(reader.read_u16::<LittleEndian>());

        let magic: HeaderMagic;

        if magic_field == HeaderMagic::LittleEndian as u16 {
            Ok(HeaderMagic::LittleEndian)
        }
        else if magic_field == HeaderMagic::BigEndian as u16 {
            Ok(HeaderMagic::BigEndian)
        } else {
            Err(Error::new(ErrorKind::Other, "Invalid magic number in header"))
        }
    }

    pub fn read_<Endian: ByteOrder>(&self, reader: &mut SeekableReader) -> Result<Box<TIFFHeader>> {

        // @todo Ensure file is >= min size

        // Bytes 4-7: offset
        // Offset from start of file to first IFD

        let ifd_offset_field = try!(reader.read_u32::<Endian>());

        // Assemble validated header

        let header = Box::new(TIFFHeader {
            byte_order: TIFFByteOrder::LittleEndian,
            magic: HeaderMagic::LittleEndian,
            ifd_offset: ifd_offset_field,
        });

        try!(reader.seek(SeekFrom::Start(ifd_offset_field as u64)));
        println!("IFD offset: {:?}", ifd_offset_field);

        try!(self.read_IFD::<Endian>(reader));

        Ok(header)
    }

    #[allow(non_snake_case)]
    fn read_IFD<Endian: ByteOrder>(&self, reader: &mut SeekableReader) -> Result<Box<IFD>> {

        // 2 byte count of IFD entries
        let entry_count = try!(reader.read_u16::<Endian>());

        println!("IFD entry count: {}", entry_count);

        let mut ifd = Box::new(IFD { count: entry_count, entries: Vec::with_capacity(entry_count as usize) });

        for entry_number in 0..entry_count as usize {
            ifd.entries.push(*self.read_tag::<Endian>(entry_number, reader).unwrap());
        }

        Ok(ifd)
    }

    fn read_tag<Endian: ByteOrder>(&self, entry_number: usize, reader: &mut SeekableReader) -> Result<Box<IFDEntry>> {
        
        // Bytes 0..1: u16 tag ID
        let tag_value = try!(reader.read_u16::<Endian>());

        // Bytes 2..3: u16 field Type
        let typ_value = try!(reader.read_u16::<Endian>());

        // Bytes 4..7: u32 number of Values of type
        let count_value = try!(reader.read_u32::<Endian>());

        // Bytes 8..11: u32 offset in file to Value
        let value_offset_value = try!(reader.read_u32::<Endian>());

        // Decode tag
        let tag_msg = format!("Invalid tag {:x}", tag_value);
        let tag = decode_tag(tag_value).expect(&tag_msg);

        // Decode type
        let typ_msg = format!("Invalid tag type {:x}", typ_value);
        let typ = decode_tag_type(typ_value).expect(&typ_msg);

        // Create entry
        let e0 = Box::new(IFDEntry {
            tag: tag,
            typ: typ,
            count: count_value,
            value_offset: value_offset_value,
        });

        println!("IFD[{:?}] tag: {:?} type: {:?} count: {} offset: {:08x}",
                 entry_number, e0.tag, e0.typ, e0.count, e0.value_offset);
        Ok(e0)
    }
}
