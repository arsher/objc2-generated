//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsiso8601dateformatoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSISO8601DateFormatOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSISO8601DateFormatOptions: NSUInteger {
        const NSISO8601DateFormatWithYear = 1;
        const NSISO8601DateFormatWithMonth = 2;
        const NSISO8601DateFormatWithWeekOfYear = 4;
        const NSISO8601DateFormatWithDay = 16;
        const NSISO8601DateFormatWithTime = 32;
        const NSISO8601DateFormatWithTimeZone = 64;
        const NSISO8601DateFormatWithSpaceBetweenDateAndTime = 128;
        const NSISO8601DateFormatWithDashSeparatorInDate = 256;
        const NSISO8601DateFormatWithColonSeparatorInTime = 512;
        const NSISO8601DateFormatWithColonSeparatorInTimeZone = 1024;
        const NSISO8601DateFormatWithFractionalSeconds = 2048;
        const NSISO8601DateFormatWithFullDate = 275;
        const NSISO8601DateFormatWithFullTime = 1632;
        const NSISO8601DateFormatWithInternetDateTime = 1907;
    }
}

unsafe impl Encode for NSISO8601DateFormatOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSISO8601DateFormatOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsiso8601dateformatter?language=objc)
    #[unsafe(super(NSFormatter, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSFormatter")]
    pub struct NSISO8601DateFormatter;
);

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCoding for NSISO8601DateFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCopying for NSISO8601DateFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl CopyingHelper for NSISO8601DateFormatter {
    type Result = Self;
}

#[cfg(feature = "NSFormatter")]
unsafe impl NSObjectProtocol for NSISO8601DateFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSSecureCoding for NSISO8601DateFormatter {}

extern_methods!(
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSISO8601DateFormatter {
        #[cfg(feature = "NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Retained<NSTimeZone>;

        #[cfg(feature = "NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[method(formatOptions)]
        pub unsafe fn formatOptions(&self) -> NSISO8601DateFormatOptions;

        #[method(setFormatOptions:)]
        pub unsafe fn setFormatOptions(&self, format_options: NSISO8601DateFormatOptions);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "NSDate", feature = "NSString"))]
        #[method_id(@__retain_semantics Other stringFromDate:)]
        pub unsafe fn stringFromDate(&self, date: &NSDate) -> Retained<NSString>;

        #[cfg(all(feature = "NSDate", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dateFromString:)]
        pub unsafe fn dateFromString(&self, string: &NSString) -> Option<Retained<NSDate>>;

        #[cfg(all(feature = "NSDate", feature = "NSString", feature = "NSTimeZone"))]
        #[method_id(@__retain_semantics Other stringFromDate:timeZone:formatOptions:)]
        pub unsafe fn stringFromDate_timeZone_formatOptions(
            date: &NSDate,
            time_zone: &NSTimeZone,
            format_options: NSISO8601DateFormatOptions,
        ) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSISO8601DateFormatter {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
