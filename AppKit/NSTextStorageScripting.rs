//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// Scripting
    #[cfg(feature = "NSTextStorage")]
    unsafe impl NSTextStorage {
        #[method_id(@__retain_semantics Other attributeRuns)]
        pub unsafe fn attributeRuns(&self) -> Retained<NSArray<NSTextStorage>>;

        /// Setter for [`attributeRuns`][Self::attributeRuns].
        #[method(setAttributeRuns:)]
        pub unsafe fn setAttributeRuns(&self, attribute_runs: &NSArray<NSTextStorage>);

        #[method_id(@__retain_semantics Other paragraphs)]
        pub unsafe fn paragraphs(&self) -> Retained<NSArray<NSTextStorage>>;

        /// Setter for [`paragraphs`][Self::paragraphs].
        #[method(setParagraphs:)]
        pub unsafe fn setParagraphs(&self, paragraphs: &NSArray<NSTextStorage>);

        #[method_id(@__retain_semantics Other words)]
        pub unsafe fn words(&self) -> Retained<NSArray<NSTextStorage>>;

        /// Setter for [`words`][Self::words].
        #[method(setWords:)]
        pub unsafe fn setWords(&self, words: &NSArray<NSTextStorage>);

        #[method_id(@__retain_semantics Other characters)]
        pub unsafe fn characters(&self) -> Retained<NSArray<NSTextStorage>>;

        /// Setter for [`characters`][Self::characters].
        #[method(setCharacters:)]
        pub unsafe fn setCharacters(&self, characters: &NSArray<NSTextStorage>);

        #[cfg(feature = "NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Retained<NSFont>>;

        #[cfg(feature = "NSFont")]
        /// Setter for [`font`][Self::font].
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other foregroundColor)]
        pub unsafe fn foregroundColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        /// Setter for [`foregroundColor`][Self::foregroundColor].
        #[method(setForegroundColor:)]
        pub unsafe fn setForegroundColor(&self, foreground_color: Option<&NSColor>);
    }
);
