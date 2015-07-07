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

#![allow(dead_code)]

//----------------------------------------------------------------------------
// Dependencies

// For binary file I/O
extern crate byteorder;

//----------------------------------------------------------------------------
// Module imports

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

use std::io::{Read, Seek};

//----------------------------------------------------------------------------
// Reexports

pub use reader::TIFFReader;

pub mod reader;
//mod writer;

//----------------------------------------------------------------------------
// Types

pub type BYTE      = u8;
pub type SHORT     = u16;
pub type LONG      = u32;
pub type ASCII     = String;
pub type RATIONAL  = (u32, u32);
pub type SBYTE     = i8;
pub type SSHORT    = i16;
pub type SLONG     = i32;
pub type SRATIONAL = (i32, i32);
pub type FLOAT     = f32;
pub type DOUBLE    = f64;

//----------------------------------------------------------------------------
// Enums

#[repr(u16)]
#[derive(Debug)]
pub enum TIFFByteOrder {
    LittleEndian = 0x4949,
    BigEndian    = 0x4d4d,
}

#[repr(u16)]
#[derive(Debug)]
pub enum HeaderMagic {
    LittleEndian = 0x002a,
    BigEndian    = 0x2a00,
}

#[repr(u16)]
#[derive(Debug,PartialEq)]
pub enum TagType {
    ByteTag           = 1,
    ASCIITag          = 2,
    ShortTag          = 3,
    LongTag           = 4,
    RationalTag       = 5,
    SignedByteTag     = 6,
    UndefinedTag      = 7,
    SignedShortTag    = 8,
    SignedLongTag     = 9,
    SignedRationalTag = 10,
    FloatTag          = 11,
    DoubleTag         = 12,

    // Not part of spec
    ShortOrLongTag    = 0xfffe,
}

#[derive(Debug)]
pub enum TagValue {
    ByteValue(BYTE),
    ShortValue(SHORT),
    LongValue(LONG),
    AsciiValue(ASCII),
    RationalValue(RATIONAL),
    SignedByteValue(SBYTE),
    SignedShortValue(SSHORT),
    SignedLongValue(SLONG),
    SignedRationalValue(SRATIONAL),
    FloatValue(FLOAT),
    DoubleValue(DOUBLE),
}

#[repr(u16)]
#[derive(Debug)]
pub enum PhotometricInterpretation {
    WhiteIsZero = 0,
    BlackIsZero = 1,
}

#[repr(u16)]
#[derive(Debug)]
pub enum Compression {
    None     = 1,
    Huffman  = 2,
    PackBits = 32773,
}

#[repr(u16)]
#[derive(Debug)]
pub enum ResolutionUnit {
    None       = 1,
    Inch       = 2,
    Centimetre = 3,
}

#[repr(u16)]
#[derive(Debug)]
pub enum SampleFormat {
    UnsignedInteger             = 1,
    TwosComplementSignedInteger = 2,
    IEEEFloatingPoint           = 3,
    Undefined                   = 4,
}

#[derive(Debug)]
pub enum ImageType {
    Bilevel,
    Grayscale,
    PaletteColour,
    RGB,
    YCbCr,
}

//----------------------------------------------------------------------------
// Structs

#[derive(Debug)]
pub struct TIFFHeader {
    pub byte_order: TIFFByteOrder,
    pub magic:      HeaderMagic,
    pub ifd_offset: LONG,
}

pub struct IFDEntry {
    tag:          TIFFTag,
    typ:          TagType,
    count:        LONG,
    value_offset: LONG,
    value:        Option<TagValue>,
}

pub struct IFD {
    count:   u16,
    entries: Vec<IFDEntry>,
}

//----------------------------------------------------------------------------
// Baseline Tags

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
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
    Predictor = 0x013d,
    ResolutionUnitTag = 0x0128,
    RowsPerStripTag = 0x0116,
    SampleFormat = 0x0153,
    SamplesPerPixel = 0x0115,
    SoftwareTag = 0x0131,
    StripByteCountsTag = 0x0117,
    StripOffsetsTag = 0x0111,
    SubfileTypeTag = 0x00ff,
    ThresholdingTag = 0x0107,
    XResolutionTag = 0x011a,
    YResolutionTag = 0x011b,

    // Section 20: Colorimetry

    WhitePointTag = 0x013e,
    PrimaryChromaticities = 0x013f,
    TransferFunction = 0x012d,
    TransferRange = 0x0156,
    ReferenceBlackWhite = 0x0214,
    
    // Section 21: YCbCr Images

    YCbCrCoefficients = 0x0211,
    YCbCrSubsampling = 0x0212,
    YCbCrPositioning = 0x0213,

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

static PHOTOMETRIC_INTERPRETATION_SHORT_DEFAULT: SHORT = 1;
static PHOTOMETRIC_INTERPRETATION_LONG_DEFAULT: LONG = 1;

//----------------------------------------------------------------------------

// Section 6: Required Fields for RGB Images

fn validate_rgb_image() {
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

pub trait SeekableReader: Seek + Read {}
impl<T: Seek + Read> SeekableReader for T {}

pub fn decode_tag(value: u16) -> Option<TIFFTag> {
    match value {
        0x013b => Some(TIFFTag::ArtistTag),
        0x0102 => Some(TIFFTag::BitsPerSampleTag),
        0x0109 => Some(TIFFTag::CellLengthTag),
        0x0108 => Some(TIFFTag::CellWidthTag),
        0x0140 => Some(TIFFTag::ColorMapTag),
        0x0103 => Some(TIFFTag::CompressionTag),
        0x8298 => Some(TIFFTag::CopyrightTag),
        0x0132 => Some(TIFFTag::DateTimeTag),
        0x0152 => Some(TIFFTag::ExtraSamplesTag),
        0x010a => Some(TIFFTag::FillOrderTag),
        0x0121 => Some(TIFFTag::FreeByteCountsTag),
        0x0120 => Some(TIFFTag::FreeOffsetsTag),
        0x0123 => Some(TIFFTag::GrayResponseCurveTag),
        0x0122 => Some(TIFFTag::GrayResponseUnitTag),
        0x013c => Some(TIFFTag::HostComputerTag),
        0x010e => Some(TIFFTag::ImageDescriptionTag),
        0x0101 => Some(TIFFTag::ImageLengthTag),
        0x0100 => Some(TIFFTag::ImageWidthTag),
        0x010f => Some(TIFFTag::MakeTag),
        0x0119 => Some(TIFFTag::MaxSampleValueTag),
        0x0118 => Some(TIFFTag::MinSampleValueTag),
        0x0110 => Some(TIFFTag::ModelTag),
        0x00fe => Some(TIFFTag::NewSubfileTypeTag),
        0x0112 => Some(TIFFTag::OrientationTag),
        0x0106 => Some(TIFFTag::PhotometricInterpretationTag),
        0x011c => Some(TIFFTag::PlanarConfigurationTag),
        0x013d => Some(TIFFTag::Predictor),
        0x0128 => Some(TIFFTag::ResolutionUnitTag),
        0x0116 => Some(TIFFTag::RowsPerStripTag),
        0x0115 => Some(TIFFTag::SamplesPerPixel),
        0x0153 => Some(TIFFTag::SampleFormat),
        0x0131 => Some(TIFFTag::SoftwareTag),
        0x0117 => Some(TIFFTag::StripByteCountsTag),
        0x0111 => Some(TIFFTag::StripOffsetsTag),
        0x00ff => Some(TIFFTag::SubfileTypeTag),
        0x0107 => Some(TIFFTag::ThresholdingTag),
        0x011a => Some(TIFFTag::XResolutionTag),
        0x011b => Some(TIFFTag::YResolutionTag),

        0x014a => Some(TIFFTag::SubIFDsTag),
        0x015b => Some(TIFFTag::JPEGTablesTag),
        0x828d => Some(TIFFTag::CFARepeatPatternDimTag),
        0x828f => Some(TIFFTag::BatteryLevelTag),
        0x83BB => Some(TIFFTag::IPTCTag),
        0x8773 => Some(TIFFTag::InterColorProfileTag),
        0x8829 => Some(TIFFTag::InterlaceTag),
        0x882a => Some(TIFFTag::TimeZoneOffsetTag),
        0x882b => Some(TIFFTag::SelfTimerModeTag),
        0x920d => Some(TIFFTag::NoiseTag),
        0x9211 => Some(TIFFTag::ImageNumberTag),
        0x9212 => Some(TIFFTag::SecurityClassificationTag),
        0x9213 => Some(TIFFTag::ImageHistoryTag),
        0x9216 => Some(TIFFTag::EPStandardIdTag),

        0x02bc => Some(TIFFTag::XMPTag),
        0x8649 => Some(TIFFTag::PhotoshopTag),
        0x8769 => Some(TIFFTag::EXIFTag),
        _ => None,
    }
}

pub fn decode_tag_type(typ: u16) -> Option<TagType> {

    match typ {
        1 => Some(TagType::ByteTag),
        2 => Some(TagType::ASCIITag),
        3 => Some(TagType::ShortTag),
        4 => Some(TagType::LongTag),
        5 => Some(TagType::RationalTag),
        6 => Some(TagType::SignedByteTag),
        7 => Some(TagType::UndefinedTag),
        8 => Some(TagType::SignedShortTag),
        9 => Some(TagType::SignedLongTag),
        10 => Some(TagType::SignedRationalTag),
        11 => Some(TagType::FloatTag),
        12 => Some(TagType::DoubleTag),
        _ => None,
    }
}

// Returns (type, count) for known tags. 0 is unknown/variable/unspecified.
pub fn type_and_count_for_tag(tag: TIFFTag) -> Option<(TagType, u32)> {
    match tag {
        TIFFTag::ArtistTag                    => Some((TagType::ASCIITag, 0)),
        TIFFTag::BitsPerSampleTag             => Some((TagType::ShortTag, 0)),
        TIFFTag::CellLengthTag                => Some((TagType::ShortTag, 1)),
        TIFFTag::CellWidthTag                 => Some((TagType::ShortTag, 1)),
        TIFFTag::ColorMapTag                  => Some((TagType::ShortTag, 0)),
        TIFFTag::CompressionTag               => Some((TagType::ShortTag, 1)),
        TIFFTag::CopyrightTag                 => Some((TagType::ASCIITag, 0)),
        TIFFTag::DateTimeTag                  => Some((TagType::ASCIITag, 0)),
        TIFFTag::ExtraSamplesTag              => Some((TagType::ShortTag, 0)),
        TIFFTag::FillOrderTag                 => Some((TagType::ShortTag, 1)),
//        TIFFTag::FreeByteCountsTag => ,
//        TIFFTag::FreeOffsetsTag => ,
        TIFFTag::GrayResponseCurveTag         => Some((TagType::ShortTag, 0)),
        TIFFTag::GrayResponseUnitTag          => Some((TagType::ShortTag, 1)),
        TIFFTag::HostComputerTag              => Some((TagType::ASCIITag, 0)),
        TIFFTag::ImageDescriptionTag          => Some((TagType::ASCIITag, 0)),
        TIFFTag::ImageLengthTag               => Some((TagType::ShortOrLongTag, 1)),
        TIFFTag::ImageWidthTag                => Some((TagType::ShortOrLongTag, 1)),
        TIFFTag::InterColorProfileTag         => Some((TagType::UndefinedTag, 0)),
        TIFFTag::MakeTag                      => Some((TagType::ASCIITag, 0)),
        TIFFTag::MaxSampleValueTag            => Some((TagType::ShortTag, 0)),
        TIFFTag::MinSampleValueTag            => Some((TagType::ShortTag, 0)),
        TIFFTag::ModelTag                     => Some((TagType::ASCIITag, 0)),
//        TIFFTag::NewSubfileTypeTag => ,
        TIFFTag::OrientationTag               => Some((TagType::ShortTag, 1)),
        TIFFTag::PhotometricInterpretationTag => Some((TagType::ShortTag, 1)),
        TIFFTag::PlanarConfigurationTag       => Some((TagType::ShortTag, 1)),
        TIFFTag::Predictor                    => Some((TagType::ShortTag, 1)),
        TIFFTag::ResolutionUnitTag            => Some((TagType::ShortTag, 1)),
        TIFFTag::RowsPerStripTag              => Some((TagType::ShortOrLongTag, 1)),
        TIFFTag::SampleFormat                 => Some((TagType::ShortTag, 0)),
        TIFFTag::SamplesPerPixel              => Some((TagType::ShortTag, 1)),
        TIFFTag::SoftwareTag                  => Some((TagType::ASCIITag, 0)),
        TIFFTag::StripByteCountsTag           => Some((TagType::ShortOrLongTag, 0)),
        TIFFTag::StripOffsetsTag              => Some((TagType::LongTag, 0)),
        TIFFTag::SubfileTypeTag               => Some((TagType::ShortTag, 1)),
        TIFFTag::ThresholdingTag              => Some((TagType::ShortTag, 1)),
        TIFFTag::XResolutionTag               => Some((TagType::RationalTag, 1)),
        TIFFTag::YResolutionTag               => Some((TagType::RationalTag, 1)),
        // Extended
        TIFFTag::XMPTag => Some((TagType::ByteTag, 0)),
        TIFFTag::PhotoshopTag => Some((TagType::ByteTag, 0)),
        TIFFTag::EXIFTag => Some((TagType::LongTag, 0)),
        //
        _ =>  None,
    }
}

/*
    Required Fields for Bilevel Images

    - ImageWidth 
    - ImageLength 
    - Compression 
    - PhotometricInterpretation 
    - StripOffsets 
    - RowsPerStrip 
    - StripByteCounts 
    - XResolution 
    - YResolution 
    - ResolutionUnit
 */

/*
    Required Fields for Grayscale Images

    - ImageWidth
    - ImageLength
    - BitsPerSample
    - Compression
    - PhotometricInterpretation
    - StripOffsets
    - RowsPerStrip
    - StripByteCounts
    - XResolution
    - YResolution
    - ResolutionUnit
 */

/*
    Required Fields for Palette Colour Images

    - ImageWidth
    - ImageLength
    - BitsPerSample
    - Compression
    - PhotometricInterpretation
    - StripOffsets
    - RowsPerStrip
    - StripByteCounts
    - XResolution
    - YResolution
    - ResolutionUnit
    - ColorMap
 */

/*
    Required Fields for RGB Images

    - ImageWidth
    - ImageLength
    - BitsPerSample
    - Compression
    - PhotometricInterpretation
    - StripOffsets
    - SamplesPerPixel
    - RowsPerStrip
    - StripByteCounts
    - XResolution
    - YResolution
    - ResolutionUnit
 */
