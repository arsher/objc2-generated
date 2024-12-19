//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// NSScripting
    #[cfg(all(feature = "NSApplication", feature = "NSResponder"))]
    unsafe impl NSApplication {
        #[cfg(feature = "NSDocument")]
        #[method_id(@__retain_semantics Other orderedDocuments)]
        pub unsafe fn orderedDocuments(&self) -> Retained<NSArray<NSDocument>>;

        #[cfg(feature = "NSWindow")]
        #[method_id(@__retain_semantics Other orderedWindows)]
        pub unsafe fn orderedWindows(&self) -> Retained<NSArray<NSWindow>>;
    }
);
