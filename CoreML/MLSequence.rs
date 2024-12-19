//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlsequence?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLSequence;
);

unsafe impl NSCoding for MLSequence {}

unsafe impl NSObjectProtocol for MLSequence {}

unsafe impl NSSecureCoding for MLSequence {}

extern_methods!(
    unsafe impl MLSequence {
        #[cfg(feature = "MLFeatureType")]
        #[method(type)]
        pub unsafe fn r#type(&self) -> MLFeatureType;

        #[cfg(feature = "MLFeatureType")]
        #[method_id(@__retain_semantics Other emptySequenceWithType:)]
        pub unsafe fn emptySequenceWithType(r#type: MLFeatureType) -> Retained<Self>;

        #[method_id(@__retain_semantics Other sequenceWithStringArray:)]
        pub unsafe fn sequenceWithStringArray(string_values: &NSArray<NSString>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other stringValues)]
        pub unsafe fn stringValues(&self) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other sequenceWithInt64Array:)]
        pub unsafe fn sequenceWithInt64Array(int64_values: &NSArray<NSNumber>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other int64Values)]
        pub unsafe fn int64Values(&self) -> Retained<NSArray<NSNumber>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLSequence {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
