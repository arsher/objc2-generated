//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlupdatecontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLUpdateContext;
);

unsafe impl NSObjectProtocol for MLUpdateContext {}

extern_methods!(
    unsafe impl MLUpdateContext {
        #[cfg(all(feature = "MLTask", feature = "MLUpdateTask"))]
        #[method_id(@__retain_semantics Other task)]
        pub unsafe fn task(&self) -> Retained<MLUpdateTask>;

        #[cfg(all(feature = "MLModel", feature = "MLWritable"))]
        #[method_id(@__retain_semantics Other model)]
        pub unsafe fn model(&self) -> Retained<MLModel>;

        #[cfg(feature = "MLUpdateProgressEvent")]
        #[method(event)]
        pub unsafe fn event(&self) -> MLUpdateProgressEvent;

        #[cfg(all(feature = "MLKey", feature = "MLMetricKey"))]
        #[method_id(@__retain_semantics Other metrics)]
        pub unsafe fn metrics(&self) -> Retained<NSDictionary<MLMetricKey, AnyObject>>;

        #[cfg(all(feature = "MLKey", feature = "MLParameterKey"))]
        #[method_id(@__retain_semantics Other parameters)]
        pub unsafe fn parameters(&self) -> Retained<NSDictionary<MLParameterKey, AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLUpdateContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
