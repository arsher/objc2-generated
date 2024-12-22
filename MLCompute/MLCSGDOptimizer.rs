//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// The MLCSGDOptimizer specifies a stochastic gradient descent optimizer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcsgdoptimizer?language=objc)
    #[unsafe(super(MLCOptimizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCOptimizer")]
    #[deprecated]
    pub struct MLCSGDOptimizer;
);

#[cfg(feature = "MLCOptimizer")]
unsafe impl NSCopying for MLCSGDOptimizer {}

#[cfg(feature = "MLCOptimizer")]
unsafe impl CopyingHelper for MLCSGDOptimizer {
    type Result = Self;
}

#[cfg(feature = "MLCOptimizer")]
unsafe impl NSObjectProtocol for MLCSGDOptimizer {}

extern_methods!(
    #[cfg(feature = "MLCOptimizer")]
    unsafe impl MLCSGDOptimizer {
        /// The momentum factor.  A hyper-parameter.
        ///
        /// The default is 0.0.
        #[deprecated]
        #[method(momentumScale)]
        pub unsafe fn momentumScale(&self) -> c_float;

        /// A boolean that specifies whether to apply nesterov momentum or not.
        ///
        /// The default is false.
        #[deprecated]
        #[method(usesNesterovMomentum)]
        pub unsafe fn usesNesterovMomentum(&self) -> bool;

        #[cfg(feature = "MLCOptimizerDescriptor")]
        /// Create an MLCSGDOptimizer object with defaults
        ///
        /// Returns: A new MLCSGDOptimizer object.
        #[deprecated]
        #[method_id(@__retain_semantics Other optimizerWithDescriptor:)]
        pub unsafe fn optimizerWithDescriptor(
            optimizer_descriptor: &MLCOptimizerDescriptor,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCOptimizerDescriptor")]
        /// Create an MLCSGDOptimizer object
        ///
        /// Parameter `optimizerDescriptor`: The optimizer descriptor object
        ///
        /// Parameter `momentumScale`: The momentum scale
        ///
        /// Parameter `usesNesterovMomentum`: A boolean to enable / disable nesterov momentum
        ///
        /// Returns: A new MLCSGDOptimizer object.
        #[deprecated]
        #[method_id(@__retain_semantics Other optimizerWithDescriptor:momentumScale:usesNesterovMomentum:)]
        pub unsafe fn optimizerWithDescriptor_momentumScale_usesNesterovMomentum(
            optimizer_descriptor: &MLCOptimizerDescriptor,
            momentum_scale: c_float,
            uses_nesterov_momentum: bool,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCOptimizer`
    #[cfg(feature = "MLCOptimizer")]
    unsafe impl MLCSGDOptimizer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
