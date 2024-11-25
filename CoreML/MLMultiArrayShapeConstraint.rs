//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmultiarrayshapeconstraint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLMultiArrayShapeConstraint;
);

unsafe impl NSCoding for MLMultiArrayShapeConstraint {}

unsafe impl NSObjectProtocol for MLMultiArrayShapeConstraint {}

unsafe impl NSSecureCoding for MLMultiArrayShapeConstraint {}

extern_methods!(
    unsafe impl MLMultiArrayShapeConstraint {
        #[cfg(feature = "MLMultiArrayShapeConstraintType")]
        #[method(type)]
        pub unsafe fn r#type(&self) -> MLMultiArrayShapeConstraintType;

        #[method_id(@__retain_semantics Other sizeRangeForDimension)]
        pub unsafe fn sizeRangeForDimension(&self) -> Retained<NSArray<NSValue>>;

        #[method_id(@__retain_semantics Other enumeratedShapes)]
        pub unsafe fn enumeratedShapes(&self) -> Retained<NSArray<NSArray<NSNumber>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLMultiArrayShapeConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
