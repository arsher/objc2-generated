//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtimevalue?language=objc)
pub type CMTimeValue = i64;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtimescale?language=objc)
pub type CMTimeScale = i32;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtimeepoch?language=objc)
pub type CMTimeEpoch = i64;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtimeflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMTimeFlags(pub u32);
bitflags::bitflags! {
    impl CMTimeFlags: u32 {
        const kCMTimeFlags_Valid = 1<<0;
        const kCMTimeFlags_HasBeenRounded = 1<<1;
        const kCMTimeFlags_PositiveInfinity = 1<<2;
        const kCMTimeFlags_NegativeInfinity = 1<<3;
        const kCMTimeFlags_Indefinite = 1<<4;
        const kCMTimeFlags_ImpliedValueFlagsMask = CMTimeFlags::kCMTimeFlags_PositiveInfinity.0|CMTimeFlags::kCMTimeFlags_NegativeInfinity.0|CMTimeFlags::kCMTimeFlags_Indefinite.0;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CMTimeFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CMTimeFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtime?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub timescale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CMTime {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <CMTimeValue>::ENCODING,
            <CMTimeScale>::ENCODING,
            <CMTimeFlags>::ENCODING,
            <CMTimeEpoch>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CMTime {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimeinvalid?language=objc)
    pub static kCMTimeInvalid: CMTime;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimeindefinite?language=objc)
    pub static kCMTimeIndefinite: CMTime;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimepositiveinfinity?language=objc)
    pub static kCMTimePositiveInfinity: CMTime;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimenegativeinfinity?language=objc)
    pub static kCMTimeNegativeInfinity: CMTime;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimezero?language=objc)
    pub static kCMTimeZero: CMTime;
}

extern "C-unwind" {
    pub fn CMTimeMake(value: i64, timescale: i32) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeMakeWithEpoch(value: i64, timescale: i32, epoch: i64) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeMakeWithSeconds(seconds: f64, preferred_timescale: i32) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeGetSeconds(time: CMTime) -> f64;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtimeroundingmethod?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMTimeRoundingMethod(pub u32);
impl CMTimeRoundingMethod {
    pub const kCMTimeRoundingMethod_RoundHalfAwayFromZero: Self = Self(1);
    pub const kCMTimeRoundingMethod_RoundTowardZero: Self = Self(2);
    pub const kCMTimeRoundingMethod_RoundAwayFromZero: Self = Self(3);
    pub const kCMTimeRoundingMethod_QuickTime: Self = Self(4);
    pub const kCMTimeRoundingMethod_RoundTowardPositiveInfinity: Self = Self(5);
    pub const kCMTimeRoundingMethod_RoundTowardNegativeInfinity: Self = Self(6);
    pub const kCMTimeRoundingMethod_Default: Self =
        Self(CMTimeRoundingMethod::kCMTimeRoundingMethod_RoundHalfAwayFromZero.0);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CMTimeRoundingMethod {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CMTimeRoundingMethod {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    pub fn CMTimeConvertScale(
        time: CMTime,
        new_timescale: i32,
        method: CMTimeRoundingMethod,
    ) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeAdd(lhs: CMTime, rhs: CMTime) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeSubtract(lhs: CMTime, rhs: CMTime) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeMultiply(time: CMTime, multiplier: i32) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeMultiplyByFloat64(time: CMTime, multiplier: f64) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeMultiplyByRatio(time: CMTime, multiplier: i32, divisor: i32) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeCompare(time1: CMTime, time2: CMTime) -> i32;
}

extern "C-unwind" {
    pub fn CMTimeMinimum(time1: CMTime, time2: CMTime) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeMaximum(time1: CMTime, time2: CMTime) -> CMTime;
}

extern "C-unwind" {
    pub fn CMTimeAbsoluteValue(time: CMTime) -> CMTime;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTimeCopyAsDictionary(time: CMTime, allocator: CFAllocatorRef) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTimeMakeFromDictionary(dictionary_representation: CFDictionaryRef) -> CMTime;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimevaluekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTimeValueKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimescalekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTimeScaleKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimeepochkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTimeEpochKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimeflagskey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTimeFlagsKey: CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTimeCopyDescription(allocator: CFAllocatorRef, time: CMTime) -> CFStringRef;
}

extern "C-unwind" {
    pub fn CMTimeShow(time: CMTime);
}
