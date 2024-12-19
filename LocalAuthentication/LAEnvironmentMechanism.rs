//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/localauthentication/laenvironmentmechanism?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LAEnvironmentMechanism;
);

unsafe impl Send for LAEnvironmentMechanism {}

unsafe impl Sync for LAEnvironmentMechanism {}

unsafe impl NSObjectProtocol for LAEnvironmentMechanism {}

extern_methods!(
    unsafe impl LAEnvironmentMechanism {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(isUsable)]
        pub unsafe fn isUsable(&self) -> bool;

        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other iconSystemName)]
        pub unsafe fn iconSystemName(&self) -> Retained<NSString>;
    }
);
