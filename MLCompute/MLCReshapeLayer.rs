//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCReshapeLayer;

    #[cfg(feature = "MLCLayer")]
    unsafe impl ClassType for MLCReshapeLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCReshapeLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCReshapeLayer {
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Id<NSArray<NSNumber>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithShape:)]
        pub unsafe fn layerWithShape(shape: &NSArray<NSNumber>) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCReshapeLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
