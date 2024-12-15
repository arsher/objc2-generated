//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphoperation?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphOperation;
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSCopying for MPSGraphOperation {}

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphOperation {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSObjectProtocol for MPSGraphOperation {}

extern_methods!(
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphOperation {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other inputTensors)]
        pub unsafe fn inputTensors(&self) -> Retained<NSArray<MPSGraphTensor>>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other outputTensors)]
        pub unsafe fn outputTensors(&self) -> Retained<NSArray<MPSGraphTensor>>;

        #[method_id(@__retain_semantics Other controlDependencies)]
        pub unsafe fn controlDependencies(&self) -> Retained<NSArray<MPSGraphOperation>>;

        #[cfg(feature = "MPSGraph")]
        #[method_id(@__retain_semantics Other graph)]
        pub unsafe fn graph(&self) -> Retained<MPSGraph>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);