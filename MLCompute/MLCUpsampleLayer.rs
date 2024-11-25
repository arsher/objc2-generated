//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcupsamplelayer?language=objc)
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCUpsampleLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCUpsampleLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCUpsampleLayer {
        #[deprecated]
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(sampleMode)]
        pub unsafe fn sampleMode(&self) -> MLCSampleMode;

        #[deprecated]
        #[method(alignsCorners)]
        pub unsafe fn alignsCorners(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithShape:)]
        pub unsafe fn layerWithShape(shape: &NSArray<NSNumber>) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithShape:sampleMode:alignsCorners:)]
        pub unsafe fn layerWithShape_sampleMode_alignsCorners(
            shape: &NSArray<NSNumber>,
            sample_mode: MLCSampleMode,
            aligns_corners: bool,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCUpsampleLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
