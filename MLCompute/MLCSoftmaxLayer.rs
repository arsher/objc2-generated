//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcsoftmaxlayer?language=objc)
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCSoftmaxLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCSoftmaxLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCSoftmaxLayer {
        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(operation)]
        pub unsafe fn operation(&self) -> MLCSoftmaxOperation;

        #[deprecated]
        #[method(dimension)]
        pub unsafe fn dimension(&self) -> NSUInteger;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithOperation:)]
        pub unsafe fn layerWithOperation(operation: MLCSoftmaxOperation) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithOperation:dimension:)]
        pub unsafe fn layerWithOperation_dimension(
            operation: MLCSoftmaxOperation,
            dimension: NSUInteger,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCSoftmaxLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
