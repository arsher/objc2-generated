//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mldictionaryconstraint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLDictionaryConstraint;
);

unsafe impl NSCoding for MLDictionaryConstraint {}

unsafe impl NSObjectProtocol for MLDictionaryConstraint {}

unsafe impl NSSecureCoding for MLDictionaryConstraint {}

extern_methods!(
    unsafe impl MLDictionaryConstraint {
        #[cfg(feature = "MLFeatureType")]
        #[method(keyType)]
        pub unsafe fn keyType(&self) -> MLFeatureType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLDictionaryConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
