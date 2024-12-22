//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A padding layer
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcpaddinglayer?language=objc)
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCPaddingLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSCopying for MLCPaddingLayer {}

#[cfg(feature = "MLCLayer")]
unsafe impl CopyingHelper for MLCPaddingLayer {
    type Result = Self;
}

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCPaddingLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCPaddingLayer {
        #[cfg(feature = "MLCTypes")]
        /// The padding type i.e. constant, zero, reflect or symmetric
        #[deprecated]
        #[method(paddingType)]
        pub unsafe fn paddingType(&self) -> MLCPaddingType;

        /// The left padding size
        #[deprecated]
        #[method(paddingLeft)]
        pub unsafe fn paddingLeft(&self) -> NSUInteger;

        /// The right padding size
        #[deprecated]
        #[method(paddingRight)]
        pub unsafe fn paddingRight(&self) -> NSUInteger;

        /// The top padding size
        #[deprecated]
        #[method(paddingTop)]
        pub unsafe fn paddingTop(&self) -> NSUInteger;

        /// The bottom padding size
        #[deprecated]
        #[method(paddingBottom)]
        pub unsafe fn paddingBottom(&self) -> NSUInteger;

        /// The constant value to use if padding type is constant.
        #[deprecated]
        #[method(constantValue)]
        pub unsafe fn constantValue(&self) -> c_float;

        /// Create a padding layer with reflection padding
        ///
        /// Parameter `padding`: The padding sizes.
        ///
        /// Returns: A new padding layer
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithReflectionPadding:)]
        pub unsafe fn layerWithReflectionPadding(padding: &NSArray<NSNumber>) -> Retained<Self>;

        /// Create a padding layer with symmetric padding
        ///
        /// Parameter `padding`: The padding sizes.
        ///
        /// Returns: A new padding layer
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithSymmetricPadding:)]
        pub unsafe fn layerWithSymmetricPadding(padding: &NSArray<NSNumber>) -> Retained<Self>;

        /// Create a padding layer with zero padding
        ///
        /// Parameter `padding`: The padding sizes.
        ///
        /// Returns: A new padding layer
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithZeroPadding:)]
        pub unsafe fn layerWithZeroPadding(padding: &NSArray<NSNumber>) -> Retained<Self>;

        /// Create a padding layer with constant padding
        ///
        /// Parameter `padding`: The padding sizes.
        ///
        /// Parameter `constantValue`: The constant value to pad the source tensor.
        ///
        /// Returns: A new padding layer
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithConstantPadding:constantValue:)]
        pub unsafe fn layerWithConstantPadding_constantValue(
            padding: &NSArray<NSNumber>,
            constant_value: c_float,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCPaddingLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
