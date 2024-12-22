//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalekey?language=objc)
// NS_TYPED_ENUM
#[cfg(feature = "NSString")]
pub type NSLocaleKey = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocale?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLocale;
);

unsafe impl Send for NSLocale {}

unsafe impl Sync for NSLocale {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSLocale {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSLocale {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSLocale {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSLocale {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSLocale {}

extern_methods!(
    unsafe impl NSLocale {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(&self, key: &NSLocaleKey) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other displayNameForKey:value:)]
        pub unsafe fn displayNameForKey_value(
            &self,
            key: &NSLocaleKey,
            value: &AnyObject,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithLocaleIdentifier:)]
        pub unsafe fn initWithLocaleIdentifier(
            this: Allocated<Self>,
            string: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// NSExtendedLocale
    unsafe impl NSLocale {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localeIdentifier)]
        pub unsafe fn localeIdentifier(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForLocaleIdentifier:)]
        pub unsafe fn localizedStringForLocaleIdentifier(
            &self,
            locale_identifier: &NSString,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other languageCode)]
        pub unsafe fn languageCode(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForLanguageCode:)]
        pub unsafe fn localizedStringForLanguageCode(
            &self,
            language_code: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        /// Returns the identifier for the language part of the locale. For example, returns "en-US" for "en_US@rg=gbzzzz"  locale.
        #[method_id(@__retain_semantics Other languageIdentifier)]
        pub unsafe fn languageIdentifier(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForCountryCode:)]
        pub unsafe fn localizedStringForCountryCode(
            &self,
            country_code: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        /// Returns the region code of the locale.
        /// If the `rg` subtag is present, the value of the subtag will be used. For example,  returns "GB" for "en_US@rg=gbzzzz" locale.
        /// If the `localeIdentifier` doesn’t contain a region, returns `nil`.
        #[method_id(@__retain_semantics Other regionCode)]
        pub unsafe fn regionCode(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other scriptCode)]
        pub unsafe fn scriptCode(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForScriptCode:)]
        pub unsafe fn localizedStringForScriptCode(
            &self,
            script_code: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other variantCode)]
        pub unsafe fn variantCode(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForVariantCode:)]
        pub unsafe fn localizedStringForVariantCode(
            &self,
            variant_code: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSCharacterSet")]
        #[method_id(@__retain_semantics Other exemplarCharacterSet)]
        pub unsafe fn exemplarCharacterSet(&self) -> Retained<NSCharacterSet>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other calendarIdentifier)]
        pub unsafe fn calendarIdentifier(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForCalendarIdentifier:)]
        pub unsafe fn localizedStringForCalendarIdentifier(
            &self,
            calendar_identifier: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other collationIdentifier)]
        pub unsafe fn collationIdentifier(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForCollationIdentifier:)]
        pub unsafe fn localizedStringForCollationIdentifier(
            &self,
            collation_identifier: &NSString,
        ) -> Option<Retained<NSString>>;

        #[method(usesMetricSystem)]
        pub unsafe fn usesMetricSystem(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other decimalSeparator)]
        pub unsafe fn decimalSeparator(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other groupingSeparator)]
        pub unsafe fn groupingSeparator(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other currencySymbol)]
        pub unsafe fn currencySymbol(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other currencyCode)]
        pub unsafe fn currencyCode(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForCurrencyCode:)]
        pub unsafe fn localizedStringForCurrencyCode(
            &self,
            currency_code: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other collatorIdentifier)]
        pub unsafe fn collatorIdentifier(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForCollatorIdentifier:)]
        pub unsafe fn localizedStringForCollatorIdentifier(
            &self,
            collator_identifier: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other quotationBeginDelimiter)]
        pub unsafe fn quotationBeginDelimiter(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other quotationEndDelimiter)]
        pub unsafe fn quotationEndDelimiter(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other alternateQuotationBeginDelimiter)]
        pub unsafe fn alternateQuotationBeginDelimiter(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other alternateQuotationEndDelimiter)]
        pub unsafe fn alternateQuotationEndDelimiter(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// NSLocaleCreation
    unsafe impl NSLocale {
        #[method_id(@__retain_semantics Other autoupdatingCurrentLocale)]
        pub unsafe fn autoupdatingCurrentLocale() -> Retained<NSLocale>;

        #[method_id(@__retain_semantics Other currentLocale)]
        pub unsafe fn currentLocale() -> Retained<NSLocale>;

        #[method_id(@__retain_semantics Other systemLocale)]
        pub unsafe fn systemLocale() -> Retained<NSLocale>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localeWithLocaleIdentifier:)]
        pub unsafe fn localeWithLocaleIdentifier(ident: &NSString) -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalelanguagedirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLocaleLanguageDirection(pub NSUInteger);
impl NSLocaleLanguageDirection {
    #[doc(alias = "NSLocaleLanguageDirectionUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "NSLocaleLanguageDirectionLeftToRight")]
    pub const LeftToRight: Self = Self(1);
    #[doc(alias = "NSLocaleLanguageDirectionRightToLeft")]
    pub const RightToLeft: Self = Self(2);
    #[doc(alias = "NSLocaleLanguageDirectionTopToBottom")]
    pub const TopToBottom: Self = Self(3);
    #[doc(alias = "NSLocaleLanguageDirectionBottomToTop")]
    pub const BottomToTop: Self = Self(4);
}

unsafe impl Encode for NSLocaleLanguageDirection {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSLocaleLanguageDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSLocaleGeneralInfo
    unsafe impl NSLocale {
        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other availableLocaleIdentifiers)]
        pub unsafe fn availableLocaleIdentifiers() -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other ISOLanguageCodes)]
        pub unsafe fn ISOLanguageCodes() -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other ISOCountryCodes)]
        pub unsafe fn ISOCountryCodes() -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other ISOCurrencyCodes)]
        pub unsafe fn ISOCurrencyCodes() -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other commonISOCurrencyCodes)]
        pub unsafe fn commonISOCurrencyCodes() -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other preferredLanguages)]
        pub unsafe fn preferredLanguages() -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other componentsFromLocaleIdentifier:)]
        pub unsafe fn componentsFromLocaleIdentifier(
            string: &NSString,
        ) -> Retained<NSDictionary<NSString, NSString>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other localeIdentifierFromComponents:)]
        pub unsafe fn localeIdentifierFromComponents(
            dict: &NSDictionary<NSString, NSString>,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other canonicalLocaleIdentifierFromString:)]
        pub unsafe fn canonicalLocaleIdentifierFromString(string: &NSString) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other canonicalLanguageIdentifierFromString:)]
        pub unsafe fn canonicalLanguageIdentifierFromString(
            string: &NSString,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localeIdentifierFromWindowsLocaleCode:)]
        pub unsafe fn localeIdentifierFromWindowsLocaleCode(
            lcid: u32,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(windowsLocaleCodeFromLocaleIdentifier:)]
        pub unsafe fn windowsLocaleCodeFromLocaleIdentifier(locale_identifier: &NSString) -> u32;

        #[cfg(feature = "NSString")]
        #[method(characterDirectionForLanguage:)]
        pub unsafe fn characterDirectionForLanguage(
            iso_lang_code: &NSString,
        ) -> NSLocaleLanguageDirection;

        #[cfg(feature = "NSString")]
        #[method(lineDirectionForLanguage:)]
        pub unsafe fn lineDirectionForLanguage(
            iso_lang_code: &NSString,
        ) -> NSLocaleLanguageDirection;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscurrentlocaledidchangenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSCurrentLocaleDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocaleidentifier?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleIdentifier: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalelanguagecode?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleLanguageCode: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalecountrycode?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleCountryCode: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalescriptcode?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleScriptCode: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalevariantcode?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleVariantCode: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocaleexemplarcharacterset?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleExemplarCharacterSet: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalecalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleCalendar: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalecollationidentifier?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleCollationIdentifier: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocaleusesmetricsystem?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleUsesMetricSystem: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalemeasurementsystem?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleMeasurementSystem: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocaledecimalseparator?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleDecimalSeparator: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalegroupingseparator?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleGroupingSeparator: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalecurrencysymbol?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleCurrencySymbol: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalecurrencycode?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleCurrencyCode: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalecollatoridentifier?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleCollatorIdentifier: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalequotationbegindelimiterkey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleQuotationBeginDelimiterKey: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalequotationenddelimiterkey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleQuotationEndDelimiterKey: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalealternatequotationbegindelimiterkey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleAlternateQuotationBeginDelimiterKey: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalealternatequotationenddelimiterkey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocaleAlternateQuotationEndDelimiterKey: &'static NSLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsgregoriancalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSGregorianCalendar: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsbuddhistcalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSBuddhistCalendar: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nschinesecalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSChineseCalendar: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshebrewcalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHebrewCalendar: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsislamiccalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIslamicCalendar: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsislamiccivilcalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIslamicCivilCalendar: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsjapanesecalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSJapaneseCalendar: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrepublicofchinacalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSRepublicOfChinaCalendar: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspersiancalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSPersianCalendar: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsindiancalendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIndianCalendar: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsiso8601calendar?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSISO8601Calendar: &'static NSString;
}
