//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCOptimizer")]
    #[deprecated]
    pub struct MLCAdamWOptimizer;

    #[cfg(feature = "MLCOptimizer")]
    unsafe impl ClassType for MLCAdamWOptimizer {
        #[inherits(NSObject)]
        type Super = MLCOptimizer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCOptimizer")]
unsafe impl NSCopying for MLCAdamWOptimizer {}

#[cfg(feature = "MLCOptimizer")]
unsafe impl NSObjectProtocol for MLCAdamWOptimizer {}

extern_methods!(
    #[cfg(feature = "MLCOptimizer")]
    unsafe impl MLCAdamWOptimizer {
        #[deprecated]
        #[method(beta1)]
        pub unsafe fn beta1(&self) -> c_float;

        #[deprecated]
        #[method(beta2)]
        pub unsafe fn beta2(&self) -> c_float;

        #[deprecated]
        #[method(epsilon)]
        pub unsafe fn epsilon(&self) -> c_float;

        #[deprecated]
        #[method(usesAMSGrad)]
        pub unsafe fn usesAMSGrad(&self) -> bool;

        #[deprecated]
        #[method(timeStep)]
        pub unsafe fn timeStep(&self) -> NSUInteger;

        #[cfg(feature = "MLCOptimizerDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other optimizerWithDescriptor:)]
        pub unsafe fn optimizerWithDescriptor(
            optimizer_descriptor: &MLCOptimizerDescriptor,
        ) -> Id<Self>;

        #[cfg(feature = "MLCOptimizerDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other optimizerWithDescriptor:beta1:beta2:epsilon:usesAMSGrad:timeStep:)]
        pub unsafe fn optimizerWithDescriptor_beta1_beta2_epsilon_usesAMSGrad_timeStep(
            optimizer_descriptor: &MLCOptimizerDescriptor,
            beta1: c_float,
            beta2: c_float,
            epsilon: c_float,
            uses_ams_grad: bool,
            time_step: NSUInteger,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCOptimizer`
    #[cfg(feature = "MLCOptimizer")]
    unsafe impl MLCAdamWOptimizer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
