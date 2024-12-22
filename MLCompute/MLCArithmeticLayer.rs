//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// An arithmetic layer
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcarithmeticlayer?language=objc)
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCArithmeticLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCArithmeticLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCArithmeticLayer {
        #[cfg(feature = "MLCTypes")]
        /// The arithmetic operation.
        #[deprecated]
        #[method(operation)]
        pub unsafe fn operation(&self) -> MLCArithmeticOperation;

        #[cfg(feature = "MLCTypes")]
        /// Create an arithmetic layer
        ///
        /// Parameter `operation`: The arithmetic operation
        ///
        /// Returns: A new arithmetic layer
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithOperation:)]
        pub unsafe fn layerWithOperation(operation: MLCArithmeticOperation) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCArithmeticLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
