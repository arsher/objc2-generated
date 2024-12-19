//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsuserinterfacecompressionoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUserInterfaceCompressionOptions;
);

unsafe impl NSCoding for NSUserInterfaceCompressionOptions {}

unsafe impl NSCopying for NSUserInterfaceCompressionOptions {}

unsafe impl CopyingHelper for NSUserInterfaceCompressionOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSUserInterfaceCompressionOptions {}

extern_methods!(
    unsafe impl NSUserInterfaceCompressionOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCompressionOptions:)]
        pub unsafe fn initWithCompressionOptions(
            this: Allocated<Self>,
            options: &NSSet<NSUserInterfaceCompressionOptions>,
        ) -> Retained<Self>;

        #[method(containsOptions:)]
        pub unsafe fn containsOptions(&self, options: &NSUserInterfaceCompressionOptions) -> bool;

        #[method(intersectsOptions:)]
        pub unsafe fn intersectsOptions(&self, options: &NSUserInterfaceCompressionOptions)
            -> bool;

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[method_id(@__retain_semantics Other optionsByAddingOptions:)]
        pub unsafe fn optionsByAddingOptions(
            &self,
            options: &NSUserInterfaceCompressionOptions,
        ) -> Retained<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other optionsByRemovingOptions:)]
        pub unsafe fn optionsByRemovingOptions(
            &self,
            options: &NSUserInterfaceCompressionOptions,
        ) -> Retained<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other hideImagesOption)]
        pub unsafe fn hideImagesOption() -> Retained<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other hideTextOption)]
        pub unsafe fn hideTextOption() -> Retained<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other reduceMetricsOption)]
        pub unsafe fn reduceMetricsOption() -> Retained<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other breakEqualWidthsOption)]
        pub unsafe fn breakEqualWidthsOption() -> Retained<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other standardOptions)]
        pub unsafe fn standardOptions() -> Retained<NSUserInterfaceCompressionOptions>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUserInterfaceCompressionOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsuserinterfacecompression?language=objc)
    pub unsafe trait NSUserInterfaceCompression {
        #[method(compressWithPrioritizedCompressionOptions:)]
        unsafe fn compressWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumSizeWithPrioritizedCompressionOptions:)]
        unsafe fn minimumSizeWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        ) -> NSSize;

        #[method_id(@__retain_semantics Other activeCompressionOptions)]
        unsafe fn activeCompressionOptions(&self) -> Retained<NSUserInterfaceCompressionOptions>;
    }

    unsafe impl ProtocolType for dyn NSUserInterfaceCompression {}
);
