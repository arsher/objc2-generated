//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvoptionflags?language=objc)
pub type CVOptionFlags = u64;

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvsmptetime?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CVSMPTETime {
    pub subframes: i16,
    pub subframeDivisor: i16,
    pub counter: u32,
    pub r#type: u32,
    pub flags: u32,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub frames: i16,
}

unsafe impl Encode for CVSMPTETime {
    const ENCODING: Encoding = Encoding::Struct(
        "CVSMPTETime",
        &[
            <i16>::ENCODING,
            <i16>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <i16>::ENCODING,
            <i16>::ENCODING,
            <i16>::ENCODING,
            <i16>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CVSMPTETime {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvsmptetimetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CVSMPTETimeType(pub u32);
impl CVSMPTETimeType {
    pub const kCVSMPTETimeType24: Self = Self(0);
    pub const kCVSMPTETimeType25: Self = Self(1);
    pub const kCVSMPTETimeType30Drop: Self = Self(2);
    pub const kCVSMPTETimeType30: Self = Self(3);
    pub const kCVSMPTETimeType2997: Self = Self(4);
    pub const kCVSMPTETimeType2997Drop: Self = Self(5);
    pub const kCVSMPTETimeType60: Self = Self(6);
    pub const kCVSMPTETimeType5994: Self = Self(7);
}

unsafe impl Encode for CVSMPTETimeType {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for CVSMPTETimeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvsmptetimeflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CVSMPTETimeFlags(pub u32);
bitflags::bitflags! {
    impl CVSMPTETimeFlags: u32 {
        const kCVSMPTETimeValid = 1<<0;
        const kCVSMPTETimeRunning = 1<<1;
    }
}

unsafe impl Encode for CVSMPTETimeFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for CVSMPTETimeFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvtimeflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CVTimeFlags(pub i32);
bitflags::bitflags! {
    impl CVTimeFlags: i32 {
        const kCVTimeIsIndefinite = 1<<0;
    }
}

unsafe impl Encode for CVTimeFlags {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for CVTimeFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvtime?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CVTime {
    pub timeValue: i64,
    pub timeScale: i32,
    pub flags: i32,
}

unsafe impl Encode for CVTime {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[<i64>::ENCODING, <i32>::ENCODING, <i32>::ENCODING]);
}

unsafe impl RefEncode for CVTime {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvtimestamp?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CVTimeStamp {
    pub version: u32,
    pub videoTimeScale: i32,
    pub videoTime: i64,
    pub hostTime: u64,
    pub rateScalar: c_double,
    pub videoRefreshPeriod: i64,
    pub smpteTime: CVSMPTETime,
    pub flags: u64,
    pub reserved: u64,
}

unsafe impl Encode for CVTimeStamp {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u32>::ENCODING,
            <i32>::ENCODING,
            <i64>::ENCODING,
            <u64>::ENCODING,
            <c_double>::ENCODING,
            <i64>::ENCODING,
            <CVSMPTETime>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CVTimeStamp {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvtimestampflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CVTimeStampFlags(pub u64);
bitflags::bitflags! {
    impl CVTimeStampFlags: u64 {
        const kCVTimeStampVideoTimeValid = 1<<0;
        const kCVTimeStampHostTimeValid = 1<<1;
        const kCVTimeStampSMPTETimeValid = 1<<2;
        const kCVTimeStampVideoRefreshPeriodValid = 1<<3;
        const kCVTimeStampRateScalarValid = 1<<4;
        const kCVTimeStampTopField = 1<<16;
        const kCVTimeStampBottomField = 1<<17;
        const kCVTimeStampVideoHostTimeValid = CVTimeStampFlags::kCVTimeStampVideoTimeValid.0|CVTimeStampFlags::kCVTimeStampHostTimeValid.0;
        const kCVTimeStampIsInterlaced = CVTimeStampFlags::kCVTimeStampTopField.0|CVTimeStampFlags::kCVTimeStampBottomField.0;
    }
}

unsafe impl Encode for CVTimeStampFlags {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for CVTimeStampFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvzerotime?language=objc)
    pub static kCVZeroTime: CVTime;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvindefinitetime?language=objc)
    pub static kCVIndefiniteTime: CVTime;
}
