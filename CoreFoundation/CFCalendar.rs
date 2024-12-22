//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfcalendarref?language=objc)
pub type CFCalendarRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCalendarGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    pub fn CFCalendarCopyCurrent() -> CFCalendarRef;
}

extern "C-unwind" {
    /// Creates a calendar.  The identifiers are the `kCF*Calendar` constants in CFLocale.h.
    #[cfg(all(feature = "CFBase", feature = "CFLocale"))]
    pub fn CFCalendarCreateWithIdentifier(
        allocator: CFAllocatorRef,
        identifier: CFCalendarIdentifier,
    ) -> CFCalendarRef;
}

extern "C-unwind" {
    /// Returns the calendar's identifier.
    #[cfg(all(feature = "CFBase", feature = "CFLocale"))]
    pub fn CFCalendarGetIdentifier(calendar: CFCalendarRef) -> CFCalendarIdentifier;
}

extern "C-unwind" {
    #[cfg(feature = "CFLocale")]
    pub fn CFCalendarCopyLocale(calendar: CFCalendarRef) -> CFLocaleRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFLocale")]
    pub fn CFCalendarSetLocale(calendar: CFCalendarRef, locale: CFLocaleRef);
}

extern "C-unwind" {
    #[cfg(feature = "CFDate")]
    pub fn CFCalendarCopyTimeZone(calendar: CFCalendarRef) -> CFTimeZoneRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFDate")]
    pub fn CFCalendarSetTimeZone(calendar: CFCalendarRef, tz: CFTimeZoneRef);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCalendarGetFirstWeekday(calendar: CFCalendarRef) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCalendarSetFirstWeekday(calendar: CFCalendarRef, wkdy: CFIndex);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCalendarGetMinimumDaysInFirstWeek(calendar: CFCalendarRef) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCalendarSetMinimumDaysInFirstWeek(calendar: CFCalendarRef, mwd: CFIndex);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfcalendarunit?language=objc)
// NS_OPTIONS
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFCalendarUnit(pub CFOptionFlags);
#[cfg(feature = "CFBase")]
bitflags::bitflags! {
    impl CFCalendarUnit: CFOptionFlags {
        const kCFCalendarUnitEra = 1<<1;
        const kCFCalendarUnitYear = 1<<2;
        const kCFCalendarUnitMonth = 1<<3;
        const kCFCalendarUnitDay = 1<<4;
        const kCFCalendarUnitHour = 1<<5;
        const kCFCalendarUnitMinute = 1<<6;
        const kCFCalendarUnitSecond = 1<<7;
#[deprecated = "Use kCFCalendarUnitWeekOfYear or kCFCalendarUnitWeekOfMonth instead"]
        const kCFCalendarUnitWeek = 1<<8;
        const kCFCalendarUnitWeekday = 1<<9;
        const kCFCalendarUnitWeekdayOrdinal = 1<<10;
        const kCFCalendarUnitQuarter = 1<<11;
        const kCFCalendarUnitWeekOfMonth = 1<<12;
        const kCFCalendarUnitWeekOfYear = 1<<13;
        const kCFCalendarUnitYearForWeekOfYear = 1<<14;
        const kCFCalendarUnitDayOfYear = 1<<16;
    }
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFCalendarUnit {
    const ENCODING: Encoding = CFOptionFlags::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFCalendarUnit {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCalendarGetMinimumRangeOfUnit(
        calendar: CFCalendarRef,
        unit: CFCalendarUnit,
    ) -> CFRange;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCalendarGetMaximumRangeOfUnit(
        calendar: CFCalendarRef,
        unit: CFCalendarUnit,
    ) -> CFRange;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFDate"))]
    pub fn CFCalendarGetRangeOfUnit(
        calendar: CFCalendarRef,
        smaller_unit: CFCalendarUnit,
        bigger_unit: CFCalendarUnit,
        at: CFAbsoluteTime,
    ) -> CFRange;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFDate"))]
    pub fn CFCalendarGetOrdinalityOfUnit(
        calendar: CFCalendarRef,
        smaller_unit: CFCalendarUnit,
        bigger_unit: CFCalendarUnit,
        at: CFAbsoluteTime,
    ) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFDate"))]
    pub fn CFCalendarGetTimeRangeOfUnit(
        calendar: CFCalendarRef,
        unit: CFCalendarUnit,
        at: CFAbsoluteTime,
        startp: *mut CFAbsoluteTime,
        tip: *mut CFTimeInterval,
    ) -> Boolean;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfcalendarcomponentswrap?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFCalendarComponentsWrap: CFOptionFlags = 1 << 0;
