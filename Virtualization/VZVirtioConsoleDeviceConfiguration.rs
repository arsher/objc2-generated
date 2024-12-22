//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Virtio Console Device
    ///
    /// This console device enables communication between the host and the guest using console ports through the Virtio interface.
    ///
    /// The device sets up one or more ports via VZVirtioConsolePortConfiguration on the Virtio console device.
    ///
    /// See: VZVirtioConsolePortConfiguration
    ///
    /// See: VZVirtualMachineConfiguration.consoleDevices
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledeviceconfiguration?language=objc)
    #[unsafe(super(VZConsoleDeviceConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZConsoleDeviceConfiguration")]
    pub struct VZVirtioConsoleDeviceConfiguration;
);

#[cfg(feature = "VZConsoleDeviceConfiguration")]
unsafe impl NSCopying for VZVirtioConsoleDeviceConfiguration {}

#[cfg(feature = "VZConsoleDeviceConfiguration")]
unsafe impl CopyingHelper for VZVirtioConsoleDeviceConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZConsoleDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZVirtioConsoleDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZConsoleDeviceConfiguration")]
    unsafe impl VZVirtioConsoleDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "VZVirtioConsolePortConfigurationArray")]
        /// The console ports to be configured for this console device.
        #[method_id(@__retain_semantics Other ports)]
        pub unsafe fn ports(&self) -> Retained<VZVirtioConsolePortConfigurationArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZConsoleDeviceConfiguration`
    #[cfg(feature = "VZConsoleDeviceConfiguration")]
    unsafe impl VZVirtioConsoleDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
