//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// The MLCActivationDescriptor specifies a neuron descriptor.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcactivationdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCActivationDescriptor;
);

unsafe impl NSCopying for MLCActivationDescriptor {}

unsafe impl CopyingHelper for MLCActivationDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCActivationDescriptor {}

extern_methods!(
    unsafe impl MLCActivationDescriptor {
        #[cfg(feature = "MLCTypes")]
        /// The type of activation function
        #[deprecated]
        #[method(activationType)]
        pub unsafe fn activationType(&self) -> MLCActivationType;

        /// Parameter to the activation function
        #[deprecated]
        #[method(a)]
        pub unsafe fn a(&self) -> c_float;

        /// Parameter to the activation function
        #[deprecated]
        #[method(b)]
        pub unsafe fn b(&self) -> c_float;

        /// Parameter to the activation function
        #[deprecated]
        #[method(c)]
        pub unsafe fn c(&self) -> c_float;

        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCActivationDescriptor object
        ///
        /// Parameter `activationType`: A type of activation function.
        ///
        /// Returns: A new neuron descriptor or nil if failure
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithType:)]
        pub unsafe fn descriptorWithType(
            activation_type: MLCActivationType,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCActivationDescriptor object
        ///
        /// Parameter `activationType`: A type of activation function.
        ///
        /// Parameter `a`: Parameter "a".
        ///
        /// Returns: A new neuron descriptor or nil if failure
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithType:a:)]
        pub unsafe fn descriptorWithType_a(
            activation_type: MLCActivationType,
            a: c_float,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCActivationDescriptor object
        ///
        /// Parameter `activationType`: A type of activation function.
        ///
        /// Parameter `a`: Parameter "a".
        ///
        /// Parameter `b`: Parameter "b".
        ///
        /// Returns: A new neuron descriptor or nil if failure
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithType:a:b:)]
        pub unsafe fn descriptorWithType_a_b(
            activation_type: MLCActivationType,
            a: c_float,
            b: c_float,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCActivationDescriptor object
        ///
        /// Parameter `activationType`: A type of activation function.
        ///
        /// Parameter `a`: Parameter "a".
        ///
        /// Parameter `b`: Parameter "b".
        ///
        /// Parameter `c`: Parameter "c".
        ///
        /// Returns: A new neuron descriptor or nil if failure
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithType:a:b:c:)]
        pub unsafe fn descriptorWithType_a_b_c(
            activation_type: MLCActivationType,
            a: c_float,
            b: c_float,
            c: c_float,
        ) -> Option<Retained<Self>>;
    }
);
