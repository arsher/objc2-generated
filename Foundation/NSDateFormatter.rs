//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdateformatterstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDateFormatterStyle(pub NSUInteger);
impl NSDateFormatterStyle {
    pub const NSDateFormatterNoStyle: Self = Self(0);
    pub const NSDateFormatterShortStyle: Self = Self(1);
    pub const NSDateFormatterMediumStyle: Self = Self(2);
    pub const NSDateFormatterLongStyle: Self = Self(3);
    pub const NSDateFormatterFullStyle: Self = Self(4);
}

unsafe impl Encode for NSDateFormatterStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDateFormatterStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdateformatterbehavior?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDateFormatterBehavior(pub NSUInteger);
impl NSDateFormatterBehavior {
    #[doc(alias = "NSDateFormatterBehaviorDefault")]
    pub const Default: Self = Self(0);
    pub const NSDateFormatterBehavior10_0: Self = Self(1000);
    pub const NSDateFormatterBehavior10_4: Self = Self(1040);
}

unsafe impl Encode for NSDateFormatterBehavior {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDateFormatterBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdateformatter?language=objc)
    #[unsafe(super(NSFormatter, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSFormatter")]
    pub struct NSDateFormatter;
);

#[cfg(feature = "NSFormatter")]
unsafe impl Send for NSDateFormatter {}

#[cfg(feature = "NSFormatter")]
unsafe impl Sync for NSDateFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCoding for NSDateFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCopying for NSDateFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl CopyingHelper for NSDateFormatter {
    type Result = Self;
}

#[cfg(feature = "NSFormatter")]
unsafe impl NSObjectProtocol for NSDateFormatter {}

extern_methods!(
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSDateFormatter {
        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formatting_context: NSFormattingContext);

        #[cfg(all(feature = "NSError", feature = "NSRange", feature = "NSString"))]
        #[method(getObjectValue:forString:range:error:_)]
        pub unsafe fn getObjectValue_forString_range_error(
            &self,
            obj: Option<&mut Option<Retained<AnyObject>>>,
            string: &NSString,
            rangep: *mut NSRange,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "NSDate", feature = "NSString"))]
        #[method_id(@__retain_semantics Other stringFromDate:)]
        pub unsafe fn stringFromDate(&self, date: &NSDate) -> Retained<NSString>;

        #[cfg(all(feature = "NSDate", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dateFromString:)]
        pub unsafe fn dateFromString(&self, string: &NSString) -> Option<Retained<NSDate>>;

        #[cfg(all(feature = "NSDate", feature = "NSString"))]
        #[method_id(@__retain_semantics Other localizedStringFromDate:dateStyle:timeStyle:)]
        pub unsafe fn localizedStringFromDate_dateStyle_timeStyle(
            date: &NSDate,
            dstyle: NSDateFormatterStyle,
            tstyle: NSDateFormatterStyle,
        ) -> Retained<NSString>;

        #[cfg(all(feature = "NSLocale", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dateFormatFromTemplate:options:locale:)]
        pub unsafe fn dateFormatFromTemplate_options_locale(
            tmplate: &NSString,
            opts: NSUInteger,
            locale: Option<&NSLocale>,
        ) -> Option<Retained<NSString>>;

        #[method(defaultFormatterBehavior)]
        pub unsafe fn defaultFormatterBehavior() -> NSDateFormatterBehavior;

        #[method(setDefaultFormatterBehavior:)]
        pub unsafe fn setDefaultFormatterBehavior(
            default_formatter_behavior: NSDateFormatterBehavior,
        );

        #[cfg(feature = "NSString")]
        #[method(setLocalizedDateFormatFromTemplate:)]
        pub unsafe fn setLocalizedDateFormatFromTemplate(&self, date_format_template: &NSString);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other dateFormat)]
        pub unsafe fn dateFormat(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method(setDateFormat:)]
        pub unsafe fn setDateFormat(&self, date_format: Option<&NSString>);

        #[method(dateStyle)]
        pub unsafe fn dateStyle(&self) -> NSDateFormatterStyle;

        #[method(setDateStyle:)]
        pub unsafe fn setDateStyle(&self, date_style: NSDateFormatterStyle);

        #[method(timeStyle)]
        pub unsafe fn timeStyle(&self) -> NSDateFormatterStyle;

        #[method(setTimeStyle:)]
        pub unsafe fn setTimeStyle(&self, time_style: NSDateFormatterStyle);

        #[cfg(feature = "NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Retained<NSLocale>;

        #[cfg(feature = "NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method(generatesCalendarDates)]
        pub unsafe fn generatesCalendarDates(&self) -> bool;

        #[method(setGeneratesCalendarDates:)]
        pub unsafe fn setGeneratesCalendarDates(&self, generates_calendar_dates: bool);

        #[method(formatterBehavior)]
        pub unsafe fn formatterBehavior(&self) -> NSDateFormatterBehavior;

        #[method(setFormatterBehavior:)]
        pub unsafe fn setFormatterBehavior(&self, formatter_behavior: NSDateFormatterBehavior);

        #[cfg(feature = "NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Retained<NSTimeZone>;

        #[cfg(feature = "NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[cfg(feature = "NSCalendar")]
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Retained<NSCalendar>;

        #[cfg(feature = "NSCalendar")]
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[method(isLenient)]
        pub unsafe fn isLenient(&self) -> bool;

        #[method(setLenient:)]
        pub unsafe fn setLenient(&self, lenient: bool);

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other twoDigitStartDate)]
        pub unsafe fn twoDigitStartDate(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method(setTwoDigitStartDate:)]
        pub unsafe fn setTwoDigitStartDate(&self, two_digit_start_date: Option<&NSDate>);

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other defaultDate)]
        pub unsafe fn defaultDate(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method(setDefaultDate:)]
        pub unsafe fn setDefaultDate(&self, default_date: Option<&NSDate>);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other eraSymbols)]
        pub unsafe fn eraSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setEraSymbols:)]
        pub unsafe fn setEraSymbols(&self, era_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other monthSymbols)]
        pub unsafe fn monthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setMonthSymbols:)]
        pub unsafe fn setMonthSymbols(&self, month_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortMonthSymbols)]
        pub unsafe fn shortMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setShortMonthSymbols:)]
        pub unsafe fn setShortMonthSymbols(&self, short_month_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other weekdaySymbols)]
        pub unsafe fn weekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setWeekdaySymbols:)]
        pub unsafe fn setWeekdaySymbols(&self, weekday_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortWeekdaySymbols)]
        pub unsafe fn shortWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setShortWeekdaySymbols:)]
        pub unsafe fn setShortWeekdaySymbols(
            &self,
            short_weekday_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other AMSymbol)]
        pub unsafe fn AMSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method(setAMSymbol:)]
        pub unsafe fn setAMSymbol(&self, am_symbol: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other PMSymbol)]
        pub unsafe fn PMSymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method(setPMSymbol:)]
        pub unsafe fn setPMSymbol(&self, pm_symbol: Option<&NSString>);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other longEraSymbols)]
        pub unsafe fn longEraSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setLongEraSymbols:)]
        pub unsafe fn setLongEraSymbols(&self, long_era_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other veryShortMonthSymbols)]
        pub unsafe fn veryShortMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setVeryShortMonthSymbols:)]
        pub unsafe fn setVeryShortMonthSymbols(
            &self,
            very_short_month_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other standaloneMonthSymbols)]
        pub unsafe fn standaloneMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setStandaloneMonthSymbols:)]
        pub unsafe fn setStandaloneMonthSymbols(
            &self,
            standalone_month_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortStandaloneMonthSymbols)]
        pub unsafe fn shortStandaloneMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setShortStandaloneMonthSymbols:)]
        pub unsafe fn setShortStandaloneMonthSymbols(
            &self,
            short_standalone_month_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other veryShortStandaloneMonthSymbols)]
        pub unsafe fn veryShortStandaloneMonthSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setVeryShortStandaloneMonthSymbols:)]
        pub unsafe fn setVeryShortStandaloneMonthSymbols(
            &self,
            very_short_standalone_month_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other veryShortWeekdaySymbols)]
        pub unsafe fn veryShortWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setVeryShortWeekdaySymbols:)]
        pub unsafe fn setVeryShortWeekdaySymbols(
            &self,
            very_short_weekday_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other standaloneWeekdaySymbols)]
        pub unsafe fn standaloneWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setStandaloneWeekdaySymbols:)]
        pub unsafe fn setStandaloneWeekdaySymbols(
            &self,
            standalone_weekday_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortStandaloneWeekdaySymbols)]
        pub unsafe fn shortStandaloneWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setShortStandaloneWeekdaySymbols:)]
        pub unsafe fn setShortStandaloneWeekdaySymbols(
            &self,
            short_standalone_weekday_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other veryShortStandaloneWeekdaySymbols)]
        pub unsafe fn veryShortStandaloneWeekdaySymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setVeryShortStandaloneWeekdaySymbols:)]
        pub unsafe fn setVeryShortStandaloneWeekdaySymbols(
            &self,
            very_short_standalone_weekday_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other quarterSymbols)]
        pub unsafe fn quarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setQuarterSymbols:)]
        pub unsafe fn setQuarterSymbols(&self, quarter_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortQuarterSymbols)]
        pub unsafe fn shortQuarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setShortQuarterSymbols:)]
        pub unsafe fn setShortQuarterSymbols(
            &self,
            short_quarter_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other standaloneQuarterSymbols)]
        pub unsafe fn standaloneQuarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setStandaloneQuarterSymbols:)]
        pub unsafe fn setStandaloneQuarterSymbols(
            &self,
            standalone_quarter_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other shortStandaloneQuarterSymbols)]
        pub unsafe fn shortStandaloneQuarterSymbols(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setShortStandaloneQuarterSymbols:)]
        pub unsafe fn setShortStandaloneQuarterSymbols(
            &self,
            short_standalone_quarter_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other gregorianStartDate)]
        pub unsafe fn gregorianStartDate(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[method(setGregorianStartDate:)]
        pub unsafe fn setGregorianStartDate(&self, gregorian_start_date: Option<&NSDate>);

        #[method(doesRelativeDateFormatting)]
        pub unsafe fn doesRelativeDateFormatting(&self) -> bool;

        #[method(setDoesRelativeDateFormatting:)]
        pub unsafe fn setDoesRelativeDateFormatting(&self, does_relative_date_formatting: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSDateFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDateFormatterCompatibility
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSDateFormatter {
        #[cfg(feature = "NSString")]
        #[deprecated = "Create an NSDateFormatter with `init` and set the dateFormat property instead."]
        #[method_id(@__retain_semantics Init initWithDateFormat:allowNaturalLanguage:)]
        pub unsafe fn initWithDateFormat_allowNaturalLanguage(
            this: Allocated<Self>,
            format: &NSString,
            flag: bool,
        ) -> Retained<Self>;

        #[deprecated = "There is no replacement"]
        #[method(allowsNaturalLanguage)]
        pub unsafe fn allowsNaturalLanguage(&self) -> bool;
    }
);
