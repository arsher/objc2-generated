//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Base class representing a directory sharing device in a virtual machine.
    ///
    /// VZDirectorySharingDevice should not be instantiated directly.
    ///
    /// Directory sharing devices are first configured on the VZVirtualMachineConfiguration through a subclass of VZDirectorySharingDeviceConfiguration.
    /// When a VZVirtualMachine is created from the configuration, the directory sharing devices are available through the VZVirtualMachine.directorySharingDevices property.
    ///
    /// The real type of VZDirectorySharingDevice corresponds to the type used by the configuration.
    /// For example, a VZVirtioFileSystemDeviceConfiguration leads to a device of type VZVirtioFileSystemDevice.
    ///
    /// See: VZVirtioFileSystemDevice
    ///
    /// See: VZVirtioFileSystemDeviceConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzdirectorysharingdevice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZDirectorySharingDevice;
);

unsafe impl NSObjectProtocol for VZDirectorySharingDevice {}

extern_methods!(
    unsafe impl VZDirectorySharingDevice {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
