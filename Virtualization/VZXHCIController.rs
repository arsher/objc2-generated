//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZUSBController")]
    pub struct VZXHCIController;

    #[cfg(feature = "VZUSBController")]
    unsafe impl ClassType for VZXHCIController {
        #[inherits(NSObject)]
        type Super = VZUSBController;
    }
);

#[cfg(feature = "VZUSBController")]
unsafe impl NSObjectProtocol for VZXHCIController {}

extern_methods!(
    #[cfg(feature = "VZUSBController")]
    unsafe impl VZXHCIController {}
);

extern_methods!(
    /// Methods declared on superclass `VZUSBController`
    #[cfg(feature = "VZUSBController")]
    unsafe impl VZXHCIController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
