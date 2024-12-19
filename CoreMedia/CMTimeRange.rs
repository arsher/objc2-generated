//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtimerange?language=objc)
#[cfg(feature = "CMTime")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMTimeRange {
    pub start: CMTime,
    pub duration: CMTime,
}

#[cfg(all(feature = "CMTime", feature = "objc2"))]
unsafe impl Encode for CMTimeRange {
    const ENCODING: Encoding = Encoding::Struct("?", &[<CMTime>::ENCODING, <CMTime>::ENCODING]);
}

#[cfg(all(feature = "CMTime", feature = "objc2"))]
unsafe impl RefEncode for CMTimeRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimerangezero?language=objc)
    #[cfg(feature = "CMTime")]
    pub static kCMTimeRangeZero: CMTimeRange;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimerangeinvalid?language=objc)
    #[cfg(feature = "CMTime")]
    pub static kCMTimeRangeInvalid: CMTimeRange;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeRangeMake(start: CMTime, duration: CMTime) -> CMTimeRange;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeRangeGetUnion(range: CMTimeRange, other_range: CMTimeRange) -> CMTimeRange;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeRangeGetIntersection(range: CMTimeRange, other_range: CMTimeRange) -> CMTimeRange;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeRangeEqual(range1: CMTimeRange, range2: CMTimeRange) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeRangeContainsTime(range: CMTimeRange, time: CMTime) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeRangeContainsTimeRange(range: CMTimeRange, other_range: CMTimeRange) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeRangeGetEnd(range: CMTimeRange) -> CMTime;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeMapTimeFromRangeToRange(
        t: CMTime,
        from_range: CMTimeRange,
        to_range: CMTimeRange,
    ) -> CMTime;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeClampToRange(time: CMTime, range: CMTimeRange) -> CMTime;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeMapDurationFromRangeToRange(
        dur: CMTime,
        from_range: CMTimeRange,
        to_range: CMTimeRange,
    ) -> CMTime;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeFoldIntoRange(time: CMTime, fold_range: CMTimeRange) -> CMTime;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeRangeFromTimeToTime(start: CMTime, end: CMTime) -> CMTimeRange;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMTime", feature = "objc2-core-foundation"))]
    pub fn CMTimeRangeCopyAsDictionary(
        range: CMTimeRange,
        allocator: CFAllocatorRef,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMTime", feature = "objc2-core-foundation"))]
    pub fn CMTimeRangeMakeFromDictionary(dictionary_representation: CFDictionaryRef)
        -> CMTimeRange;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimerangestartkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTimeRangeStartKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimerangedurationkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTimeRangeDurationKey: CFStringRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMTime", feature = "objc2-core-foundation"))]
    pub fn CMTimeRangeCopyDescription(allocator: CFAllocatorRef, range: CMTimeRange)
        -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeRangeShow(range: CMTimeRange);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtimemapping?language=objc)
#[cfg(feature = "CMTime")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMTimeMapping {
    pub source: CMTimeRange,
    pub target: CMTimeRange,
}

#[cfg(all(feature = "CMTime", feature = "objc2"))]
unsafe impl Encode for CMTimeMapping {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[<CMTimeRange>::ENCODING, <CMTimeRange>::ENCODING]);
}

#[cfg(all(feature = "CMTime", feature = "objc2"))]
unsafe impl RefEncode for CMTimeMapping {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimemappinginvalid?language=objc)
    #[cfg(feature = "CMTime")]
    pub static kCMTimeMappingInvalid: CMTimeMapping;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeMappingMake(source: CMTimeRange, target: CMTimeRange) -> CMTimeMapping;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeMappingMakeEmpty(target: CMTimeRange) -> CMTimeMapping;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMTime", feature = "objc2-core-foundation"))]
    pub fn CMTimeMappingCopyAsDictionary(
        mapping: CMTimeMapping,
        allocator: CFAllocatorRef,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMTime", feature = "objc2-core-foundation"))]
    pub fn CMTimeMappingMakeFromDictionary(
        dictionary_representation: CFDictionaryRef,
    ) -> CMTimeMapping;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimemappingsourcekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTimeMappingSourceKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtimemappingtargetkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTimeMappingTargetKey: CFStringRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMTime", feature = "objc2-core-foundation"))]
    pub fn CMTimeMappingCopyDescription(
        allocator: CFAllocatorRef,
        mapping: CMTimeMapping,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CMTime")]
    pub fn CMTimeMappingShow(mapping: CMTimeMapping);
}
