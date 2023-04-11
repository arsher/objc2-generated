//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDateFormatterStyle {
        NSDateFormatterNoStyle = 0,
        NSDateFormatterShortStyle = 1,
        NSDateFormatterMediumStyle = 2,
        NSDateFormatterLongStyle = 3,
        NSDateFormatterFullStyle = 4,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDateFormatterBehavior {
        NSDateFormatterBehaviorDefault = 0,
        NSDateFormatterBehavior10_0 = 1000,
        NSDateFormatterBehavior10_4 = 1040,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDateFormatter")]
    pub struct NSDateFormatter;

    #[cfg(feature = "Foundation_NSDateFormatter")]
    unsafe impl ClassType for NSDateFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSDateFormatter")]
unsafe impl NSCoding for NSDateFormatter {}

#[cfg(feature = "Foundation_NSDateFormatter")]
unsafe impl NSCopying for NSDateFormatter {}

#[cfg(feature = "Foundation_NSDateFormatter")]
unsafe impl NSObjectProtocol for NSDateFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSDateFormatter")]
    unsafe impl NSDateFormatter {
        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formatting_context: NSFormattingContext);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(getObjectValue:forString:range:error:_)]
        pub unsafe fn getObjectValue_forString_range_error(
            &self,
            obj: Option<&mut Option<Id<Object>>>,
            string: &NSString,
            rangep: *mut NSRange,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromDate:)]
        pub unsafe fn stringFromDate(&self, date: &NSDate) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dateFromString:)]
        pub unsafe fn dateFromString(&self, string: &NSString) -> Option<Id<NSDate>>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedStringFromDate:dateStyle:timeStyle:)]
        pub unsafe fn localizedStringFromDate_dateStyle_timeStyle(
            date: &NSDate,
            dstyle: NSDateFormatterStyle,
            tstyle: NSDateFormatterStyle,
        ) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSLocale", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dateFormatFromTemplate:options:locale:)]
        pub unsafe fn dateFormatFromTemplate_options_locale(
            tmplate: &NSString,
            opts: NSUInteger,
            locale: Option<&NSLocale>,
        ) -> Option<Id<NSString>>;

        #[method(defaultFormatterBehavior)]
        pub unsafe fn defaultFormatterBehavior() -> NSDateFormatterBehavior;

        #[method(setDefaultFormatterBehavior:)]
        pub unsafe fn setDefaultFormatterBehavior(
            default_formatter_behavior: NSDateFormatterBehavior,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedDateFormatFromTemplate:)]
        pub unsafe fn setLocalizedDateFormatFromTemplate(&self, date_format_template: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dateFormat)]
        pub unsafe fn dateFormat(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
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

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSLocale")]
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

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Id<NSCalendar>;

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[method(isLenient)]
        pub unsafe fn isLenient(&self) -> bool;

        #[method(setLenient:)]
        pub unsafe fn setLenient(&self, lenient: bool);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other twoDigitStartDate)]
        pub unsafe fn twoDigitStartDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setTwoDigitStartDate:)]
        pub unsafe fn setTwoDigitStartDate(&self, two_digit_start_date: Option<&NSDate>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other defaultDate)]
        pub unsafe fn defaultDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setDefaultDate:)]
        pub unsafe fn setDefaultDate(&self, default_date: Option<&NSDate>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other eraSymbols)]
        pub unsafe fn eraSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setEraSymbols:)]
        pub unsafe fn setEraSymbols(&self, era_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other monthSymbols)]
        pub unsafe fn monthSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setMonthSymbols:)]
        pub unsafe fn setMonthSymbols(&self, month_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other shortMonthSymbols)]
        pub unsafe fn shortMonthSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setShortMonthSymbols:)]
        pub unsafe fn setShortMonthSymbols(&self, short_month_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other weekdaySymbols)]
        pub unsafe fn weekdaySymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setWeekdaySymbols:)]
        pub unsafe fn setWeekdaySymbols(&self, weekday_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other shortWeekdaySymbols)]
        pub unsafe fn shortWeekdaySymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setShortWeekdaySymbols:)]
        pub unsafe fn setShortWeekdaySymbols(
            &self,
            short_weekday_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other AMSymbol)]
        pub unsafe fn AMSymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAMSymbol:)]
        pub unsafe fn setAMSymbol(&self, am_symbol: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other PMSymbol)]
        pub unsafe fn PMSymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPMSymbol:)]
        pub unsafe fn setPMSymbol(&self, pm_symbol: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other longEraSymbols)]
        pub unsafe fn longEraSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setLongEraSymbols:)]
        pub unsafe fn setLongEraSymbols(&self, long_era_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other veryShortMonthSymbols)]
        pub unsafe fn veryShortMonthSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setVeryShortMonthSymbols:)]
        pub unsafe fn setVeryShortMonthSymbols(
            &self,
            very_short_month_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other standaloneMonthSymbols)]
        pub unsafe fn standaloneMonthSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setStandaloneMonthSymbols:)]
        pub unsafe fn setStandaloneMonthSymbols(
            &self,
            standalone_month_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other shortStandaloneMonthSymbols)]
        pub unsafe fn shortStandaloneMonthSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setShortStandaloneMonthSymbols:)]
        pub unsafe fn setShortStandaloneMonthSymbols(
            &self,
            short_standalone_month_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other veryShortStandaloneMonthSymbols)]
        pub unsafe fn veryShortStandaloneMonthSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setVeryShortStandaloneMonthSymbols:)]
        pub unsafe fn setVeryShortStandaloneMonthSymbols(
            &self,
            very_short_standalone_month_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other veryShortWeekdaySymbols)]
        pub unsafe fn veryShortWeekdaySymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setVeryShortWeekdaySymbols:)]
        pub unsafe fn setVeryShortWeekdaySymbols(
            &self,
            very_short_weekday_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other standaloneWeekdaySymbols)]
        pub unsafe fn standaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setStandaloneWeekdaySymbols:)]
        pub unsafe fn setStandaloneWeekdaySymbols(
            &self,
            standalone_weekday_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other shortStandaloneWeekdaySymbols)]
        pub unsafe fn shortStandaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setShortStandaloneWeekdaySymbols:)]
        pub unsafe fn setShortStandaloneWeekdaySymbols(
            &self,
            short_standalone_weekday_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other veryShortStandaloneWeekdaySymbols)]
        pub unsafe fn veryShortStandaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setVeryShortStandaloneWeekdaySymbols:)]
        pub unsafe fn setVeryShortStandaloneWeekdaySymbols(
            &self,
            very_short_standalone_weekday_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other quarterSymbols)]
        pub unsafe fn quarterSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setQuarterSymbols:)]
        pub unsafe fn setQuarterSymbols(&self, quarter_symbols: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other shortQuarterSymbols)]
        pub unsafe fn shortQuarterSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setShortQuarterSymbols:)]
        pub unsafe fn setShortQuarterSymbols(
            &self,
            short_quarter_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other standaloneQuarterSymbols)]
        pub unsafe fn standaloneQuarterSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setStandaloneQuarterSymbols:)]
        pub unsafe fn setStandaloneQuarterSymbols(
            &self,
            standalone_quarter_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other shortStandaloneQuarterSymbols)]
        pub unsafe fn shortStandaloneQuarterSymbols(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setShortStandaloneQuarterSymbols:)]
        pub unsafe fn setShortStandaloneQuarterSymbols(
            &self,
            short_standalone_quarter_symbols: Option<&NSArray<NSString>>,
        );

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other gregorianStartDate)]
        pub unsafe fn gregorianStartDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setGregorianStartDate:)]
        pub unsafe fn setGregorianStartDate(&self, gregorian_start_date: Option<&NSDate>);

        #[method(doesRelativeDateFormatting)]
        pub unsafe fn doesRelativeDateFormatting(&self) -> bool;

        #[method(setDoesRelativeDateFormatting:)]
        pub unsafe fn setDoesRelativeDateFormatting(&self, does_relative_date_formatting: bool);
    }
);

extern_methods!(
    /// NSDateFormatterCompatibility
    #[cfg(feature = "Foundation_NSDateFormatter")]
    unsafe impl NSDateFormatter {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Create an NSDateFormatter with `init` and set the dateFormat property instead."]
        #[method_id(@__retain_semantics Init initWithDateFormat:allowNaturalLanguage:)]
        pub unsafe fn initWithDateFormat_allowNaturalLanguage(
            this: Option<Allocated<Self>>,
            format: &NSString,
            flag: bool,
        ) -> Id<Self>;

        #[deprecated = "There is no replacement"]
        #[method(allowsNaturalLanguage)]
        pub unsafe fn allowsNaturalLanguage(&self) -> bool;
    }
);
