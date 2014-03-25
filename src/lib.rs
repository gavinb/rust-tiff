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

#[crate_id = "tiff#0.1"];
#[desc = "TIFF Support"];
#[crate_type = "rlib"];
#[crate_type = "dylib"];
#[license = "MIT"];

#[allow(dead_code)];

//use std::io::{IoResult, IoError, Reader, Seek, SeekCur, SeekSet};
//use std::io::fs::File;
use std::vec_ng::Vec;
//use std::option::{Option, Some, None};

pub mod reader;
//mod writer;

//----------------------------------------------------------------------------
// Types

pub type BYTE = u8;
pub type SHORT = u16;
pub type LONG = u32;
// pub type ASCII = &[u8];
pub type RATIONAL = (u32, u32);
pub type SBYTE = i8;
pub type SSHORT = i16;
pub type SLONG = i32;
pub type SRATIONAL = (i32, i32);
pub type FLOAT = f32;
pub type DOUBLE = f64;

//----------------------------------------------------------------------------
// Enums

#[repr(u16)]
pub enum ByteOrder {
    ByteOrderLittleEndian = 0x4949,
    ByteOrderBigEndian = 0x4d4d,
}

#[repr(u16)]
pub enum HeaderMagic {
    HeaderMagicLittleEndian = 0x002a,
    HeaderMagicBigEndian = 0x2a00,
}

#[repr(u16)]
#[deriving(Show)]
pub enum TagType {
    ByteTag = 1,
    ASCIITag = 2,
    ShortTag = 3,
    LongTag = 4,
    RationalTag = 5,
    SByteTag = 6,
    UndefinedTag = 7,
    SShortTag = 8,
    SLongTag = 9,
    SRationalTag = 10,
    FLoatTag = 11,
    DoubleTag = 12,
}

pub enum PhotometricInterpretation {
    PhotometricInterpretationWhiteIsZero = 0,
    PhotometricInterpretationBlackIsZero = 1,
}

pub enum Compression {
    CompressionNone = 1,
    CompressionHuffman = 2,
    CompressionPackBits = 32773,
}

pub enum ResolutionUnit {
    ResolutionUnitNone = 1,
    ResolutionUnitInch = 2,
    ResolutionUnitCentimetre = 3,
}

//----------------------------------------------------------------------------
// Structs

pub struct TIFFHeader {
    byteOrder: ByteOrder,
    magic: HeaderMagic,
    ifdOffset: LONG,
}

pub struct IFDEntry {
    tag: TIFFTag,
    typ: TagType,
    count: LONG,
    valueOffset: LONG,
}

pub struct IFD {
    count: u16,
    entries: Vec<IFDEntry>,
}

//----------------------------------------------------------------------------
// Baseline Tags

#[repr(u16)]
#[deriving(Show)]
pub enum TIFFTag {

    // Baseline Tags

    ArtistTag = 0x013b,
    BitsPerSampleTag = 0x0102,
    CellLengthTag = 0x0109,
    CellWidthTag = 0x0108,
    ColorMapTag = 0x0140,
    CompressionTag = 0x0103,
    CopyrightTag = 0x8298,
    DateTimeTag = 0x0132,
    ExtraSamplesTag = 0x0152,
    FillOrderTag = 0x010a,
    FreeByteCountsTag = 0x0121,
    FreeOffsetsTag = 0x0120,
    GrayResponseCurveTag = 0x0123,
    GrayResponseUnitTag = 0x0122,
    HostComputerTag = 0x013c,
    ImageDescriptionTag = 0x010e,
    ImageLengthTag = 0x0101,
    ImageWidthTag = 0x0100,
    MakeTag = 0x010f,
    MaxSampleValueTag = 0x0119,
    MinSampleValueTag = 0x0118,
    ModelTag = 0x0110,
    NewSubfileTypeTag = 0x00fe,
    OrientationTag = 0x0112,
    PhotometricInterpretationTag = 0x0106,
    PlanarConfigurationTag = 0x011c,
    ResolutionUnitTag = 0x0128,
    RowsPerStripTag = 0x0116,
    SamplesPerPixel = 0x0115,
    SoftwareTag = 0x0131,
    StripByteCountsTag = 0x0117,
    StripOffsetsTag = 0x0111,
    SubfileTypeTag = 0x00ff,
    ThresholdingTag = 0x0107,
    XResolutionTag = 0x011a,
    YResolutionTag = 0x011b,

    // TIFF/EP Tags

    SubIFDsTag = 0x014a,
    JPEGTablesTag = 0x015b,
    CFARepeatPatternDimTag = 0x828d,
    BatteryLevelTag = 0x828f,
    IPTCTag = 0x83BB,
    InterColorProfileTag = 0x8773,
    InterlaceTag = 0x8829,
    TimeZoneOffsetTag = 0x882a,
    SelfTimerModeTag = 0x882b,
    NoiseTag = 0x920d,
    ImageNumberTag = 0x9211,
    SecurityClassificationTag = 0x9212,
    ImageHistoryTag = 0x9213,
    EPStandardIdTag = 0x9216,

    // Extension TIFF Tags
    // See http://www.awaresystems.be/imaging/tiff/tifftags/extension.html
    XMPTag = 0x02bc,

    // Private Tags
    PhotoshopTag = 0x8649,
    EXIFTag = 0x8769,
}

//----------------------------------------------------------------------------
// Default Values

static PhotometricInterpretationShortDefault: SHORT = 1;
static PhotometricInterpretationLongDefault: LONG = 1;

//----------------------------------------------------------------------------

// Section 6: Required Fields for RGB Images

fn validateRGBImage() {
    // ImageWidth
    // ImageLength
    // BitsPerSample
    // Compression
    // PhotometricInterpretation
    // StripOffsets
    // SamplesPerPixel
    // RowsPerStrip
    // StripByteCounts
    // XResolution
    // YResolution
    // ResolutionUnit
}

//----------------------------------------------------------------------------

pub trait SeekableReader: Seek + Reader {}

pub fn decodeTag(value: u16) -> Option<TIFFTag> {
    match value {
        0x013b => Some(ArtistTag),
        0x0102 => Some(BitsPerSampleTag),
        0x0109 => Some(CellLengthTag),
        0x0108 => Some(CellWidthTag),
        0x0140 => Some(ColorMapTag),
        0x0103 => Some(CompressionTag),
        0x8298 => Some(CopyrightTag),
        0x0132 => Some(DateTimeTag),
        0x0152 => Some(ExtraSamplesTag),
        0x010a => Some(FillOrderTag),
        0x0121 => Some(FreeByteCountsTag),
        0x0120 => Some(FreeOffsetsTag),
        0x0123 => Some(GrayResponseCurveTag),
        0x0122 => Some(GrayResponseUnitTag),
        0x013c => Some(HostComputerTag),
        0x010e => Some(ImageDescriptionTag),
        0x0101 => Some(ImageLengthTag),
        0x0100 => Some(ImageWidthTag),
        0x010f => Some(MakeTag),
        0x0119 => Some(MaxSampleValueTag),
        0x0118 => Some(MinSampleValueTag),
        0x0110 => Some(ModelTag),
        0x00fe => Some(NewSubfileTypeTag),
        0x0112 => Some(OrientationTag),
        0x0106 => Some(PhotometricInterpretationTag),
        0x011c => Some(PlanarConfigurationTag),
        0x0128 => Some(ResolutionUnitTag),
        0x0116 => Some(RowsPerStripTag),
        0x0115 => Some(SamplesPerPixel),
        0x0131 => Some(SoftwareTag),
        0x0117 => Some(StripByteCountsTag),
        0x0111 => Some(StripOffsetsTag),
        0x00ff => Some(SubfileTypeTag),
        0x0107 => Some(ThresholdingTag),
        0x011a => Some(XResolutionTag),
        0x011b => Some(YResolutionTag),

        0x014a => Some(SubIFDsTag),
        0x015b => Some(JPEGTablesTag),
        0x828d => Some(CFARepeatPatternDimTag),
        0x828f => Some(BatteryLevelTag),
        0x83BB => Some(IPTCTag),
        0x8773 => Some(InterColorProfileTag),
        0x8829 => Some(InterlaceTag),
        0x882a => Some(TimeZoneOffsetTag),
        0x882b => Some(SelfTimerModeTag),
        0x920d => Some(NoiseTag),
        0x9211 => Some(ImageNumberTag),
        0x9212 => Some(SecurityClassificationTag),
        0x9213 => Some(ImageHistoryTag),
        0x9216 => Some(EPStandardIdTag),

        0x02bc => Some(XMPTag),
        0x8649 => Some(PhotoshopTag),
        0x8769 => Some(EXIFTag),
        _ => None,
    }
}

pub fn decodeTagType(typ: u16) -> Option<TagType> {

    match typ {
        1 => Some(ByteTag),
        2 => Some(ASCIITag),
        3 => Some(ShortTag),
        4 => Some(LongTag),
        5 => Some(RationalTag),
        6 => Some(SByteTag),
        7 => Some(UndefinedTag),
        8 => Some(SShortTag),
        9 => Some(SLongTag),
        10 => Some(SRationalTag),
        11 => Some(FLoatTag),
        12 => Some(DoubleTag),
        _ => None,
    }
}
