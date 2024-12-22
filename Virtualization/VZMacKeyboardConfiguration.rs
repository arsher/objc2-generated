//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Configuration for a Mac keyboard.
    ///
    /// This device can be used by VZVirtualMachineView to send keyboard events to the virtual machine.
    /// This keyboard supports Apple-specific features such as the globe key.
    /// Note: this device is only recognized by virtual machines running macOS 13.0 and later. In order to support both macOS 13.0 and earlier
    /// guests, VZVirtualMachineConfiguration.keyboards can be set to an array containing both a VZMacKeyboardConfiguration and
    /// a VZUSBKeyboardConfiguration object. macOS 13.0 and later guests will use the Mac keyboard device,
    /// while earlier versions of macOS will use the USB keyboard device.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmackeyboardconfiguration?language=objc)
    #[unsafe(super(VZKeyboardConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZKeyboardConfiguration")]
    pub struct VZMacKeyboardConfiguration;
);

#[cfg(feature = "VZKeyboardConfiguration")]
unsafe impl NSCopying for VZMacKeyboardConfiguration {}

#[cfg(feature = "VZKeyboardConfiguration")]
unsafe impl CopyingHelper for VZMacKeyboardConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZKeyboardConfiguration")]
unsafe impl NSObjectProtocol for VZMacKeyboardConfiguration {}

extern_methods!(
    #[cfg(feature = "VZKeyboardConfiguration")]
    unsafe impl VZMacKeyboardConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZKeyboardConfiguration`
    #[cfg(feature = "VZKeyboardConfiguration")]
    unsafe impl VZMacKeyboardConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
