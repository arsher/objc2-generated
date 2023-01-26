//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(u64)]
    pub enum NSTextCheckingType {
        NSTextCheckingTypeOrthography = 1 << 0,
        NSTextCheckingTypeSpelling = 1 << 1,
        NSTextCheckingTypeGrammar = 1 << 2,
        NSTextCheckingTypeDate = 1 << 3,
        NSTextCheckingTypeAddress = 1 << 4,
        NSTextCheckingTypeLink = 1 << 5,
        NSTextCheckingTypeQuote = 1 << 6,
        NSTextCheckingTypeDash = 1 << 7,
        NSTextCheckingTypeReplacement = 1 << 8,
        NSTextCheckingTypeCorrection = 1 << 9,
        NSTextCheckingTypeRegularExpression = 1 << 10,
        NSTextCheckingTypePhoneNumber = 1 << 11,
        NSTextCheckingTypeTransitInformation = 1 << 12,
    }
);

ns_enum!(
    #[underlying(u64)]
    pub enum NSTextCheckingTypes {
        NSTextCheckingAllSystemTypes = 0xffffffff,
        NSTextCheckingAllCustomTypes = 0xffffffff << 32,
        NSTextCheckingAllTypes = NSTextCheckingAllSystemTypes | NSTextCheckingAllCustomTypes,
    }
);

typed_extensible_enum!(
    pub type NSTextCheckingKey = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSTextCheckingResult")]
    pub struct NSTextCheckingResult;

    #[cfg(feature = "Foundation_NSTextCheckingResult")]
    unsafe impl ClassType for NSTextCheckingResult {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSTextCheckingResult")]
unsafe impl NSCoding for NSTextCheckingResult {}

#[cfg(feature = "Foundation_NSTextCheckingResult")]
unsafe impl NSObjectProtocol for NSTextCheckingResult {}

#[cfg(feature = "Foundation_NSTextCheckingResult")]
unsafe impl NSSecureCoding for NSTextCheckingResult {}

extern_methods!(
    #[cfg(feature = "Foundation_NSTextCheckingResult")]
    unsafe impl NSTextCheckingResult {
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSTextCheckingType;

        #[method(range)]
        pub unsafe fn range(&self) -> NSRange;
    }
);

extern_methods!(
    /// NSTextCheckingResultOptional
    #[cfg(feature = "Foundation_NSTextCheckingResult")]
    unsafe impl NSTextCheckingResult {
        #[cfg(feature = "Foundation_NSOrthography")]
        #[method_id(@__retain_semantics Other orthography)]
        pub unsafe fn orthography(&self) -> Option<Id<NSOrthography, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other grammarDetails)]
        pub unsafe fn grammarDetails(
            &self,
        ) -> Option<Id<NSArray<NSDictionary<NSString, Object>>, Shared>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Option<Id<NSDate, Shared>>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone, Shared>>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other components)]
        pub unsafe fn components(
            &self,
        ) -> Option<Id<NSDictionary<NSTextCheckingKey, NSString>, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other replacementString)]
        pub unsafe fn replacementString(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other alternativeStrings)]
        pub unsafe fn alternativeStrings(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[cfg(feature = "Foundation_NSRegularExpression")]
        #[method_id(@__retain_semantics Other regularExpression)]
        pub unsafe fn regularExpression(&self) -> Option<Id<NSRegularExpression, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneNumber)]
        pub unsafe fn phoneNumber(&self) -> Option<Id<NSString, Shared>>;

        #[method(numberOfRanges)]
        pub unsafe fn numberOfRanges(&self) -> NSUInteger;

        #[method(rangeAtIndex:)]
        pub unsafe fn rangeAtIndex(&self, idx: NSUInteger) -> NSRange;

        #[cfg(feature = "Foundation_NSString")]
        #[method(rangeWithName:)]
        pub unsafe fn rangeWithName(&self, name: &NSString) -> NSRange;

        #[method_id(@__retain_semantics Other resultByAdjustingRangesWithOffset:)]
        pub unsafe fn resultByAdjustingRangesWithOffset(
            &self,
            offset: NSInteger,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other addressComponents)]
        pub unsafe fn addressComponents(
            &self,
        ) -> Option<Id<NSDictionary<NSTextCheckingKey, NSString>, Shared>>;
    }
);

extern_static!(NSTextCheckingNameKey: &'static NSTextCheckingKey);

extern_static!(NSTextCheckingJobTitleKey: &'static NSTextCheckingKey);

extern_static!(NSTextCheckingOrganizationKey: &'static NSTextCheckingKey);

extern_static!(NSTextCheckingStreetKey: &'static NSTextCheckingKey);

extern_static!(NSTextCheckingCityKey: &'static NSTextCheckingKey);

extern_static!(NSTextCheckingStateKey: &'static NSTextCheckingKey);

extern_static!(NSTextCheckingZIPKey: &'static NSTextCheckingKey);

extern_static!(NSTextCheckingCountryKey: &'static NSTextCheckingKey);

extern_static!(NSTextCheckingPhoneKey: &'static NSTextCheckingKey);

extern_static!(NSTextCheckingAirlineKey: &'static NSTextCheckingKey);

extern_static!(NSTextCheckingFlightKey: &'static NSTextCheckingKey);

extern_methods!(
    /// NSTextCheckingResultCreation
    #[cfg(feature = "Foundation_NSTextCheckingResult")]
    unsafe impl NSTextCheckingResult {
        #[cfg(feature = "Foundation_NSOrthography")]
        #[method_id(@__retain_semantics Other orthographyCheckingResultWithRange:orthography:)]
        pub unsafe fn orthographyCheckingResultWithRange_orthography(
            range: NSRange,
            orthography: &NSOrthography,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[method_id(@__retain_semantics Other spellCheckingResultWithRange:)]
        pub unsafe fn spellCheckingResultWithRange(
            range: NSRange,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other grammarCheckingResultWithRange:details:)]
        pub unsafe fn grammarCheckingResultWithRange_details(
            range: NSRange,
            details: &NSArray<NSDictionary<NSString, Object>>,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other dateCheckingResultWithRange:date:)]
        pub unsafe fn dateCheckingResultWithRange_date(
            range: NSRange,
            date: &NSDate,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSTimeZone"))]
        #[method_id(@__retain_semantics Other dateCheckingResultWithRange:date:timeZone:duration:)]
        pub unsafe fn dateCheckingResultWithRange_date_timeZone_duration(
            range: NSRange,
            date: &NSDate,
            time_zone: &NSTimeZone,
            duration: NSTimeInterval,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other addressCheckingResultWithRange:components:)]
        pub unsafe fn addressCheckingResultWithRange_components(
            range: NSRange,
            components: &NSDictionary<NSTextCheckingKey, NSString>,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other linkCheckingResultWithRange:URL:)]
        pub unsafe fn linkCheckingResultWithRange_URL(
            range: NSRange,
            url: &NSURL,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other quoteCheckingResultWithRange:replacementString:)]
        pub unsafe fn quoteCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dashCheckingResultWithRange:replacementString:)]
        pub unsafe fn dashCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other replacementCheckingResultWithRange:replacementString:)]
        pub unsafe fn replacementCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other correctionCheckingResultWithRange:replacementString:)]
        pub unsafe fn correctionCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other correctionCheckingResultWithRange:replacementString:alternativeStrings:)]
        pub unsafe fn correctionCheckingResultWithRange_replacementString_alternativeStrings(
            range: NSRange,
            replacement_string: &NSString,
            alternative_strings: &NSArray<NSString>,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(feature = "Foundation_NSRegularExpression")]
        #[method_id(@__retain_semantics Other regularExpressionCheckingResultWithRanges:count:regularExpression:)]
        pub unsafe fn regularExpressionCheckingResultWithRanges_count_regularExpression(
            ranges: NSRangePointer,
            count: NSUInteger,
            regular_expression: &NSRegularExpression,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneNumberCheckingResultWithRange:phoneNumber:)]
        pub unsafe fn phoneNumberCheckingResultWithRange_phoneNumber(
            range: NSRange,
            phone_number: &NSString,
        ) -> Id<NSTextCheckingResult, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other transitInformationCheckingResultWithRange:components:)]
        pub unsafe fn transitInformationCheckingResultWithRange_components(
            range: NSRange,
            components: &NSDictionary<NSTextCheckingKey, NSString>,
        ) -> Id<NSTextCheckingResult, Shared>;
    }
);
