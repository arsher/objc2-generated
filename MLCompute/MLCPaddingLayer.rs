//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcpaddinglayer?language=objc)
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
        #[deprecated]
        #[method(paddingType)]
        pub unsafe fn paddingType(&self) -> MLCPaddingType;

        #[deprecated]
        #[method(paddingLeft)]
        pub unsafe fn paddingLeft(&self) -> NSUInteger;

        #[deprecated]
        #[method(paddingRight)]
        pub unsafe fn paddingRight(&self) -> NSUInteger;

        #[deprecated]
        #[method(paddingTop)]
        pub unsafe fn paddingTop(&self) -> NSUInteger;

        #[deprecated]
        #[method(paddingBottom)]
        pub unsafe fn paddingBottom(&self) -> NSUInteger;

        #[deprecated]
        #[method(constantValue)]
        pub unsafe fn constantValue(&self) -> c_float;

        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithReflectionPadding:)]
        pub unsafe fn layerWithReflectionPadding(padding: &NSArray<NSNumber>) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithSymmetricPadding:)]
        pub unsafe fn layerWithSymmetricPadding(padding: &NSArray<NSNumber>) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithZeroPadding:)]
        pub unsafe fn layerWithZeroPadding(padding: &NSArray<NSNumber>) -> Retained<Self>;

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
