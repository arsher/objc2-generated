//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphtensor?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphTensor;
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSCopying for MPSGraphTensor {}

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphTensor {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSObjectProtocol for MPSGraphTensor {}

extern_methods!(
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphTensor {
        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Option<Retained<MPSShape>>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MPSDataType;

        #[cfg(feature = "MPSGraphOperation")]
        #[method_id(@__retain_semantics Other operation)]
        pub unsafe fn operation(&self) -> Retained<MPSGraphOperation>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphTensor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);