//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCMatMulDescriptor;

    unsafe impl ClassType for MLCMatMulDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MLCMatMulDescriptor {}

unsafe impl NSObjectProtocol for MLCMatMulDescriptor {}

extern_methods!(
    unsafe impl MLCMatMulDescriptor {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[deprecated]
        #[method(alpha)]
        pub unsafe fn alpha(&self) -> c_float;

        #[deprecated]
        #[method(transposesX)]
        pub unsafe fn transposesX(&self) -> bool;

        #[deprecated]
        #[method(transposesY)]
        pub unsafe fn transposesY(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithAlpha:transposesX:transposesY:)]
        pub unsafe fn descriptorWithAlpha_transposesX_transposesY(
            alpha: c_float,
            transposes_x: bool,
            transposes_y: bool,
        ) -> Option<Id<Self>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor() -> Id<Self>;
    }
);