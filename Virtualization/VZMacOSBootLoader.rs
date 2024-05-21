//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZBootLoader")]
    pub struct VZMacOSBootLoader;

    #[cfg(feature = "VZBootLoader")]
    unsafe impl ClassType for VZMacOSBootLoader {
        #[inherits(NSObject)]
        type Super = VZBootLoader;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZBootLoader")]
unsafe impl NSCopying for VZMacOSBootLoader {}

#[cfg(feature = "VZBootLoader")]
unsafe impl NSObjectProtocol for VZMacOSBootLoader {}

extern_methods!(
    #[cfg(feature = "VZBootLoader")]
    unsafe impl VZMacOSBootLoader {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZBootLoader`
    #[cfg(feature = "VZBootLoader")]
    unsafe impl VZMacOSBootLoader {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
