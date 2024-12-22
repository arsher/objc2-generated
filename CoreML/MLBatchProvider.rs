//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// Protocol for accessing a collection of feature providers
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlbatchprovider?language=objc)
    pub unsafe trait MLBatchProvider {
        /// Total number of feature providers
        #[method(count)]
        unsafe fn count(&self) -> NSInteger;

        #[cfg(feature = "MLFeatureProvider")]
        /// Indexed access to collection
        #[method_id(@__retain_semantics Other featuresAtIndex:)]
        unsafe fn featuresAtIndex(
            &self,
            index: NSInteger,
        ) -> Retained<ProtocolObject<dyn MLFeatureProvider>>;
    }

    unsafe impl ProtocolType for dyn MLBatchProvider {}
);
