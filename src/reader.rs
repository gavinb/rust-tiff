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

//#[feature(globs)];

use std::io::{IoResult, IoError, Reader, Seek, SeekCur, SeekSet};
use std::io::fs::File;
use std::vec_ng::Vec;
use std::option::{Option, Some, None};

use {ByteOrder, ByteOrderBigEndian, ByteOrderLittleEndian, HeaderMagic, HeaderMagicLittleEndian, HeaderMagicBigEndian, TIFFHeader, IFD, IFDEntry, decodeTag, decodeTagType};

pub struct TIFFReader;

impl TIFFReader {

    pub fn load(&self, filename: &str) -> IoResult<u32> {

        let filepath = Path::new(filename);
        let mut reader = File::open(&filepath);

//        self.read(&reader.unwrap())
//    }
//
//    fn read(&self, reader: &SeekableReader) -> IoResult<u32> {
//
        // @todo Ensure file is >= min size

        // Read and validate ByteOrder

        let byteOrderField = try!(reader.read_le_u16());
        let byteOrder: ByteOrder;

        if byteOrderField == ByteOrderLittleEndian as u16 {
            byteOrder = ByteOrderLittleEndian;
        } else if byteOrderField == ByteOrderBigEndian as u16 {
            byteOrder = ByteOrderBigEndian;
        } else {
            fail!("Invalid byteOrder");
        }

        // Read and validate HeaderMagic

        let magicField = try!(reader.read_le_u16());
        let magic: HeaderMagic;

        if magicField == HeaderMagicLittleEndian as u16 {
            magic = HeaderMagicLittleEndian;
        }
        else if magicField == HeaderMagicBigEndian as u16 {
            magic = HeaderMagicBigEndian;
        } else {
            fail!("Invalid magic");
        }

        // Read offset to first IFD

        let ifdOffsetField = try!(reader.read_le_u32());

        // Assemble validated header

        let header = TIFFHeader {
            byteOrder: byteOrder,
            magic: magic,
            ifdOffset: ifdOffsetField,
        };

        try!(reader.seek(ifdOffsetField as i64, SeekSet));
        println!("IFD offset: {}", ifdOffsetField);

//        self.readIFD(reader);
//
//        Ok(42)
//    }
//
//    fn readIFD(&self, reader: &mut SeekableReader) -> IoResult<~IFD> {
//
        let entryCount = try!(reader.read_le_u16());

        let ifd = ~IFD { count: entryCount, entries: Vec::with_capacity(entryCount as uint) };

        println!("IFD entry count: {}", entryCount);

        for entryNumber in range(0, entryCount) {

            let tagValue = try!(reader.read_le_u16());
            let typValue = try!(reader.read_le_u16());
            let countValue = try!(reader.read_le_u32());
            let valueOffsetValue = try!(reader.read_le_u32());

            let tag = decodeTag(tagValue).expect(format!("Invalid tag {:x}", tagValue));
            let typ = decodeTagType(typValue).expect(format!("Invalid tag type {:x}", typValue));

            let e0 = IFDEntry {
                tag: tag,
                typ: typ,
                count: countValue,
                valueOffset: valueOffsetValue,
            };

            println!("IFD[{}] {} {} {:x} {}", entryNumber, e0.tag, e0.typ, e0.count, e0.valueOffset);
        }

        Ok(42)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_tiff() {
    }
}
