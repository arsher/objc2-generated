//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslistformatter?language=objc)
    #[unsafe(super(NSFormatter, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSFormatter")]
    pub struct NSListFormatter;
);

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCoding for NSListFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCopying for NSListFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl CopyingHelper for NSListFormatter {
    type Result = Self;
}

#[cfg(feature = "NSFormatter")]
unsafe impl NSObjectProtocol for NSListFormatter {}

extern_methods!(
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSListFormatter {
        #[cfg(feature = "NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Retained<NSLocale>;

        #[cfg(feature = "NSLocale")]
        /// Setter for [`locale`][Self::locale].
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method_id(@__retain_semantics Other itemFormatter)]
        pub unsafe fn itemFormatter(&self) -> Option<Retained<NSFormatter>>;

        /// Setter for [`itemFormatter`][Self::itemFormatter].
        #[method(setItemFormatter:)]
        pub unsafe fn setItemFormatter(&self, item_formatter: Option<&NSFormatter>);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other localizedStringByJoiningStrings:)]
        pub unsafe fn localizedStringByJoiningStrings(
            strings: &NSArray<NSString>,
        ) -> Retained<NSString>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other stringFromItems:)]
        pub unsafe fn stringFromItems(&self, items: &NSArray) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&AnyObject>,
        ) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSListFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
