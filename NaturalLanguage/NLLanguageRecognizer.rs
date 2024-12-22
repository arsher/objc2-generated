//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/naturallanguage/nllanguagerecognizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NLLanguageRecognizer;
);

unsafe impl NSObjectProtocol for NLLanguageRecognizer {}

extern_methods!(
    unsafe impl NLLanguageRecognizer {
        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other dominantLanguageForString:)]
        pub unsafe fn dominantLanguageForString(string: &NSString) -> Option<Retained<NLLanguage>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(processString:)]
        pub unsafe fn processString(&self, string: &NSString);

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other dominantLanguage)]
        pub unsafe fn dominantLanguage(&self) -> Option<Retained<NLLanguage>>;

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other languageHypothesesWithMaximum:)]
        pub unsafe fn languageHypothesesWithMaximum(
            &self,
            max_hypotheses: NSUInteger,
        ) -> Retained<NSDictionary<NLLanguage, NSNumber>>;

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other languageHints)]
        pub unsafe fn languageHints(&self) -> Retained<NSDictionary<NLLanguage, NSNumber>>;

        #[cfg(feature = "NLLanguage")]
        /// Setter for [`languageHints`][Self::languageHints].
        #[method(setLanguageHints:)]
        pub unsafe fn setLanguageHints(&self, language_hints: &NSDictionary<NLLanguage, NSNumber>);

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other languageConstraints)]
        pub unsafe fn languageConstraints(&self) -> Retained<NSArray<NLLanguage>>;

        #[cfg(feature = "NLLanguage")]
        /// Setter for [`languageConstraints`][Self::languageConstraints].
        #[method(setLanguageConstraints:)]
        pub unsafe fn setLanguageConstraints(&self, language_constraints: &NSArray<NLLanguage>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NLLanguageRecognizer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
