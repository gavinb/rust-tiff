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

use {TIFFByteOrder, HeaderMagic, TIFFHeader, IFD, IFDEntry, decode_tag, decode_tag_type, type_and_count_for_tag, SeekableReader, BYTE, SBYTE, SHORT, SSHORT, LONG, SLONG, FLOAT, TagType, TagValue};

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
            let entry = self.read_tag::<Endian>(entry_number, reader);
            match entry {
                Ok(e) => ifd.entries.push(e),
                Err(err) => println!("Invalid tag at index {}: {}", entry_number, err),
            }
        }

        Ok(ifd)
    }

    fn read_tag<Endian: ByteOrder>(&self, entry_number: usize, reader: &mut SeekableReader) -> Result<IFDEntry> {
        
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
        let mut e0 = IFDEntry {
            tag: tag,
            typ: typ,
            count: count_value,
            value_offset: value_offset_value,
            value: None,
        };

        let maybe_tac = type_and_count_for_tag(e0.tag);

        if maybe_tac.is_none() {
            return Err(Error::new(ErrorKind::Other,
                                  format!("Unknown tag {:?} in IFD", e0.tag)));
        }

        let (expected_typ, expected_count) = maybe_tac.unwrap();

        println!("IFD[{:?}] tag: {:?} type: {:?} count: {} offset: {:08x}",
                 entry_number, e0.tag, e0.typ, e0.count, e0.value_offset);

        let valid_short_or_long = expected_typ == TagType::ShortOrLongTag &&
            (e0.typ == TagType::ShortTag ||
             e0.typ == TagType::LongTag);

        if  ! valid_short_or_long && e0.typ != expected_typ {
            println!("    *** ERROR: expected typ: {:?} found: {:?}", expected_typ, e0.typ);
        }

        if expected_count != 0 && e0.count != expected_count {
            println!("    *** ERROR: expected count: {:?} found: {:?}", expected_count, e0.count);
        }

        /*
            p15: Value/Offset

            To save time and space the Value Offset contains the Value instead
            of pointing to the Value if and only if the Value fits into 4
            bytes. If the Value is shorter than 4 bytes, it is left-justified
            within the 4-byte Value Offset, i.e., stored in the lower- numbered
            bytes. Whether the Value fits within 4 bytes is determined by the
            Type and Count of the field.
        */

        // Try to read values
        if e0.count == 1 {
            e0.value = match e0.typ {
                TagType::ByteTag => Some(TagValue::ByteValue(e0.value_offset as BYTE)),
                TagType::ShortTag => Some(TagValue::ShortValue(e0.value_offset as SHORT)),
                TagType::LongTag => Some(TagValue::LongValue(e0.value_offset)),
                TagType::SignedByteTag => Some(TagValue::SignedByteValue(e0.value_offset as SBYTE)),
                TagType::SignedShortTag => Some(TagValue::SignedShortValue(e0.value_offset as SSHORT)),
                TagType::SignedLongTag => Some(TagValue::SignedLongValue(e0.value_offset as SLONG)),
                TagType::FloatTag => Some(TagValue::FloatValue(e0.value_offset as FLOAT)),
                TagType::ShortOrLongTag => Some(TagValue::LongValue(e0.value_offset as LONG)), // @todo FIXME
                _ => None
            };
        }

        println!("    {:?}", e0.value);

        Ok(e0)
    }
}
