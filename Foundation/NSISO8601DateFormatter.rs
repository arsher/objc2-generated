//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSISO8601DateFormatOptions {
        NSISO8601DateFormatWithYear = 1,
        NSISO8601DateFormatWithMonth = 2,
        NSISO8601DateFormatWithWeekOfYear = 4,
        NSISO8601DateFormatWithDay = 16,
        NSISO8601DateFormatWithTime = 32,
        NSISO8601DateFormatWithTimeZone = 64,
        NSISO8601DateFormatWithSpaceBetweenDateAndTime = 128,
        NSISO8601DateFormatWithDashSeparatorInDate = 256,
        NSISO8601DateFormatWithColonSeparatorInTime = 512,
        NSISO8601DateFormatWithColonSeparatorInTimeZone = 1024,
        NSISO8601DateFormatWithFractionalSeconds = 2048,
        NSISO8601DateFormatWithFullDate = 275,
        NSISO8601DateFormatWithFullTime = 1632,
        NSISO8601DateFormatWithInternetDateTime = 1907,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSISO8601DateFormatter")]
    pub struct NSISO8601DateFormatter;

    #[cfg(feature = "Foundation_NSISO8601DateFormatter")]
    unsafe impl ClassType for NSISO8601DateFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
    }
);

#[cfg(feature = "Foundation_NSISO8601DateFormatter")]
unsafe impl NSCoding for NSISO8601DateFormatter {}

#[cfg(feature = "Foundation_NSISO8601DateFormatter")]
unsafe impl NSObjectProtocol for NSISO8601DateFormatter {}

#[cfg(feature = "Foundation_NSISO8601DateFormatter")]
unsafe impl NSSecureCoding for NSISO8601DateFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSISO8601DateFormatter")]
    unsafe impl NSISO8601DateFormatter {
        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[method(formatOptions)]
        pub unsafe fn formatOptions(&self) -> NSISO8601DateFormatOptions;

        #[method(setFormatOptions:)]
        pub unsafe fn setFormatOptions(&self, format_options: NSISO8601DateFormatOptions);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromDate:)]
        pub unsafe fn stringFromDate(&self, date: &NSDate) -> Id<NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dateFromString:)]
        pub unsafe fn dateFromString(&self, string: &NSString) -> Option<Id<NSDate, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSString",
            feature = "Foundation_NSTimeZone"
        ))]
        #[method_id(@__retain_semantics Other stringFromDate:timeZone:formatOptions:)]
        pub unsafe fn stringFromDate_timeZone_formatOptions(
            date: &NSDate,
            time_zone: &NSTimeZone,
            format_options: NSISO8601DateFormatOptions,
        ) -> Id<NSString, Shared>;
    }
);
