//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZGraphicsDevice")]
    pub struct VZVirtioGraphicsDevice;

    #[cfg(feature = "VZGraphicsDevice")]
    unsafe impl ClassType for VZVirtioGraphicsDevice {
        #[inherits(NSObject)]
        type Super = VZGraphicsDevice;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZGraphicsDevice")]
unsafe impl NSObjectProtocol for VZVirtioGraphicsDevice {}

extern_methods!(
    #[cfg(feature = "VZGraphicsDevice")]
    unsafe impl VZVirtioGraphicsDevice {}
);

extern_methods!(
    /// Methods declared on superclass `VZGraphicsDevice`
    #[cfg(feature = "VZGraphicsDevice")]
    unsafe impl VZVirtioGraphicsDevice {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);