//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsorthography?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSOrthography;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSOrthography {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSOrthography {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSOrthography {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSOrthography {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSOrthography {}

extern_methods!(
    unsafe impl NSOrthography {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other dominantScript)]
        pub unsafe fn dominantScript(&self) -> Retained<NSString>;

        #[cfg(all(feature = "NSArray", feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other languageMap)]
        pub unsafe fn languageMap(&self) -> Retained<NSDictionary<NSString, NSArray<NSString>>>;

        #[cfg(all(feature = "NSArray", feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithDominantScript:languageMap:)]
        pub unsafe fn initWithDominantScript_languageMap(
            this: Allocated<Self>,
            script: &NSString,
            map: &NSDictionary<NSString, NSArray<NSString>>,
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
    /// Methods declared on superclass `NSObject`
    unsafe impl NSOrthography {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSOrthographyExtended
    unsafe impl NSOrthography {
        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other languagesForScript:)]
        pub unsafe fn languagesForScript(
            &self,
            script: &NSString,
        ) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other dominantLanguageForScript:)]
        pub unsafe fn dominantLanguageForScript(
            &self,
            script: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other dominantLanguage)]
        pub unsafe fn dominantLanguage(&self) -> Retained<NSString>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other allScripts)]
        pub unsafe fn allScripts(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other allLanguages)]
        pub unsafe fn allLanguages(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other defaultOrthographyForLanguage:)]
        pub unsafe fn defaultOrthographyForLanguage(language: &NSString) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSOrthographyCreation
    unsafe impl NSOrthography {
        #[cfg(all(feature = "NSArray", feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other orthographyWithDominantScript:languageMap:)]
        pub unsafe fn orthographyWithDominantScript_languageMap(
            script: &NSString,
            map: &NSDictionary<NSString, NSArray<NSString>>,
        ) -> Retained<Self>;
    }
);
