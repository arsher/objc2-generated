//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLMultiArrayConstraint;

    unsafe impl ClassType for MLMultiArrayConstraint {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MLMultiArrayConstraint {}

unsafe impl NSObjectProtocol for MLMultiArrayConstraint {}

unsafe impl NSSecureCoding for MLMultiArrayConstraint {}

extern_methods!(
    unsafe impl MLMultiArrayConstraint {
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "MLMultiArray")]
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MLMultiArrayDataType;

        #[cfg(feature = "MLMultiArrayShapeConstraint")]
        #[method_id(@__retain_semantics Other shapeConstraint)]
        pub unsafe fn shapeConstraint(&self) -> Retained<MLMultiArrayShapeConstraint>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLMultiArrayConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
