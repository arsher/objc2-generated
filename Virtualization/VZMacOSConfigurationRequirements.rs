//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// VZMacOSConfigurationRequirements describes the parameter constraints required by a specific configuration of macOS.
    ///
    /// When a VZMacOSRestoreImage is loaded, it can be inspected to determine the configurations supported by that restore image.
    ///
    /// See also: VZMacHardwareModel
    ///
    /// See also: VZMacOSRestoreImage
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmacosconfigurationrequirements?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMacOSConfigurationRequirements;
);

unsafe impl NSObjectProtocol for VZMacOSConfigurationRequirements {}

extern_methods!(
    unsafe impl VZMacOSConfigurationRequirements {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "VZMacHardwareModel")]
        /// The hardware model for this configuration.
        ///
        /// The hardware model can be used to configure a new virtual machine that meets the requirements.
        /// Use VZMacPlatformConfiguration.hardwareModel to configure the Mac platform, and -[VZMacAuxiliaryStorage initCreatingStorageAtURL:hardwareModel:options:error:] to create its auxiliary storage.
        ///
        /// See also: VZMacPlatformConfiguration
        ///
        /// See also: VZMacAuxiliaryStorage
        #[method_id(@__retain_semantics Other hardwareModel)]
        pub unsafe fn hardwareModel(&self) -> Retained<VZMacHardwareModel>;

        /// The minimum supported number of CPUs for this configuration.
        ///
        /// A VZMacOSConfigurationRequirements object is associated with a specific VZMacOSRestoreImage object, and thus a specific macOS configuration.
        /// This property specifies the minimum number of CPUs required by the associated macOS configuration.
        /// Installing or running the associated configuration of macOS on a virtual machine with fewer than this number of CPUs will result in undefined behavior.
        #[method(minimumSupportedCPUCount)]
        pub unsafe fn minimumSupportedCPUCount(&self) -> NSUInteger;

        /// The minimum supported memory size for this configuration.
        ///
        /// A VZMacOSConfigurationRequirements object is associated with a specific VZMacOSRestoreImage object, and thus a specific macOS configuration.
        /// This property specifies the minimum amount of memory required by the associated macOS configuration.
        /// Installing or running the associated configuration of macOS on a virtual machine with less than this amount of memory will result in undefined behavior.
        #[method(minimumSupportedMemorySize)]
        pub unsafe fn minimumSupportedMemorySize(&self) -> u64;
    }
);
