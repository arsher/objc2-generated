//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cflocaleidentifier?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "CFBase")]
pub type CFLocaleIdentifier = CFStringRef;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cflocalekey?language=objc)
// NS_TYPED_ENUM
#[cfg(feature = "CFBase")]
pub type CFLocaleKey = CFStringRef;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cflocaleref?language=objc)
pub type CFLocaleRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    pub fn CFLocaleGetSystem() -> CFLocaleRef;
}

extern "C-unwind" {
    pub fn CFLocaleCopyCurrent() -> CFLocaleRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFArray")]
    pub fn CFLocaleCopyAvailableLocaleIdentifiers() -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFArray")]
    pub fn CFLocaleCopyISOLanguageCodes() -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFArray")]
    pub fn CFLocaleCopyISOCountryCodes() -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFArray")]
    pub fn CFLocaleCopyISOCurrencyCodes() -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFArray")]
    pub fn CFLocaleCopyCommonISOCurrencyCodes() -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFArray")]
    pub fn CFLocaleCopyPreferredLanguages() -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleCreateCanonicalLanguageIdentifierFromString(
        allocator: CFAllocatorRef,
        locale_identifier: CFStringRef,
    ) -> CFLocaleIdentifier;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleCreateCanonicalLocaleIdentifierFromString(
        allocator: CFAllocatorRef,
        locale_identifier: CFStringRef,
    ) -> CFLocaleIdentifier;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleCreateCanonicalLocaleIdentifierFromScriptManagerCodes(
        allocator: CFAllocatorRef,
        lcode: LangCode,
        rcode: RegionCode,
    ) -> CFLocaleIdentifier;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleCreateLocaleIdentifierFromWindowsLocaleCode(
        allocator: CFAllocatorRef,
        lcid: u32,
    ) -> CFLocaleIdentifier;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleGetWindowsLocaleCodeFromLocaleIdentifier(
        locale_identifier: CFLocaleIdentifier,
    ) -> u32;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cflocalelanguagedirection?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFLocaleLanguageDirection(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFLocaleLanguageDirection {
    pub const kCFLocaleLanguageDirectionUnknown: Self = Self(0);
    pub const kCFLocaleLanguageDirectionLeftToRight: Self = Self(1);
    pub const kCFLocaleLanguageDirectionRightToLeft: Self = Self(2);
    pub const kCFLocaleLanguageDirectionTopToBottom: Self = Self(3);
    pub const kCFLocaleLanguageDirectionBottomToTop: Self = Self(4);
}

#[cfg(feature = "CFBase")]
unsafe impl Encode for CFLocaleLanguageDirection {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(feature = "CFBase")]
unsafe impl RefEncode for CFLocaleLanguageDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleGetLanguageCharacterDirection(
        iso_lang_code: CFStringRef,
    ) -> CFLocaleLanguageDirection;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleGetLanguageLineDirection(
        iso_lang_code: CFStringRef,
    ) -> CFLocaleLanguageDirection;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFDictionary"))]
    pub fn CFLocaleCreateComponentsFromLocaleIdentifier(
        allocator: CFAllocatorRef,
        locale_id: CFLocaleIdentifier,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFDictionary"))]
    pub fn CFLocaleCreateLocaleIdentifierFromComponents(
        allocator: CFAllocatorRef,
        dictionary: CFDictionaryRef,
    ) -> CFLocaleIdentifier;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleCreate(
        allocator: CFAllocatorRef,
        locale_identifier: CFLocaleIdentifier,
    ) -> CFLocaleRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleCreateCopy(allocator: CFAllocatorRef, locale: CFLocaleRef) -> CFLocaleRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleGetIdentifier(locale: CFLocaleRef) -> CFLocaleIdentifier;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleGetValue(locale: CFLocaleRef, key: CFLocaleKey) -> CFTypeRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFLocaleCopyDisplayNameForPropertyValue(
        display_locale: CFLocaleRef,
        key: CFLocaleKey,
        value: CFStringRef,
    ) -> CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalecurrentlocaledidchangenotification?language=objc)
    #[cfg(all(feature = "CFBase", feature = "CFNotificationCenter"))]
    pub static kCFLocaleCurrentLocaleDidChangeNotification: CFNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocaleidentifier?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleIdentifier: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalelanguagecode?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleLanguageCode: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalecountrycode?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleCountryCode: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalescriptcode?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleScriptCode: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalevariantcode?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleVariantCode: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocaleexemplarcharacterset?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleExemplarCharacterSet: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalecalendaridentifier?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleCalendarIdentifier: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalecalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleCalendar: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalecollationidentifier?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleCollationIdentifier: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocaleusesmetricsystem?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleUsesMetricSystem: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalemeasurementsystem?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleMeasurementSystem: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocaledecimalseparator?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleDecimalSeparator: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalegroupingseparator?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleGroupingSeparator: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalecurrencysymbol?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleCurrencySymbol: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalecurrencycode?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleCurrencyCode: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalecollatoridentifier?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleCollatorIdentifier: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalequotationbegindelimiterkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleQuotationBeginDelimiterKey: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalequotationenddelimiterkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleQuotationEndDelimiterKey: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalealternatequotationbegindelimiterkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleAlternateQuotationBeginDelimiterKey: CFLocaleKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcflocalealternatequotationenddelimiterkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFLocaleAlternateQuotationEndDelimiterKey: CFLocaleKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfcalendaridentifier?language=objc)
// NS_TYPED_ENUM
#[cfg(feature = "CFBase")]
pub type CFCalendarIdentifier = CFStringRef;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfgregoriancalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFGregorianCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfbuddhistcalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFBuddhistCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfchinesecalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFChineseCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfhebrewcalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFHebrewCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfislamiccalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFIslamicCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfislamiccivilcalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFIslamicCivilCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfjapanesecalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFJapaneseCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfrepublicofchinacalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFRepublicOfChinaCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfpersiancalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFPersianCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfindiancalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFIndianCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfiso8601calendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFISO8601Calendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfislamictabularcalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFIslamicTabularCalendar: CFCalendarIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfislamicummalquracalendar?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFIslamicUmmAlQuraCalendar: CFCalendarIdentifier;
}
