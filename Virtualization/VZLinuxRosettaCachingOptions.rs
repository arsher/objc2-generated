//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Base class for a VZLinuxRosettaCachingOptions.
    ///
    /// VZLinuxRosettaCachingOptions define the communication mechanism between the Rosetta daemon and the Rosetta runtime.
    ///
    /// VZLinuxRosettaCachingOptions should not be instantiated directly.
    /// One of its subclasses like VZLinuxRosettaUnixSocketCachingOptions or VZLinuxRosettaAbstractCachingOptions should be used instead.
    ///
    ///
    /// See: VZLinuxRosettaUnixSocketCachingOptions
    ///
    /// See: VZLinuxRosettaAbstractCachingOptions
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzlinuxrosettacachingoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZLinuxRosettaCachingOptions;
);

unsafe impl NSObjectProtocol for VZLinuxRosettaCachingOptions {}

extern_methods!(
    unsafe impl VZLinuxRosettaCachingOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
