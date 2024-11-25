//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzerrordomain?language=objc)
    pub static VZErrorDomain: Option<&'static NSErrorDomain>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzerrorcode?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VZErrorCode(pub NSInteger);
impl VZErrorCode {
    pub const VZErrorInternal: Self = Self(1);
    pub const VZErrorInvalidVirtualMachineConfiguration: Self = Self(2);
    pub const VZErrorInvalidVirtualMachineState: Self = Self(3);
    pub const VZErrorInvalidVirtualMachineStateTransition: Self = Self(4);
    pub const VZErrorInvalidDiskImage: Self = Self(5);
    pub const VZErrorVirtualMachineLimitExceeded: Self = Self(6);
    pub const VZErrorNetworkError: Self = Self(7);
    pub const VZErrorOutOfDiskSpace: Self = Self(8);
    pub const VZErrorOperationCancelled: Self = Self(9);
    pub const VZErrorNotSupported: Self = Self(10);
    pub const VZErrorSave: Self = Self(11);
    pub const VZErrorRestore: Self = Self(12);
    pub const VZErrorRestoreImageCatalogLoadFailed: Self = Self(10001);
    pub const VZErrorInvalidRestoreImageCatalog: Self = Self(10002);
    pub const VZErrorNoSupportedRestoreImagesInCatalog: Self = Self(10003);
    pub const VZErrorRestoreImageLoadFailed: Self = Self(10004);
    pub const VZErrorInvalidRestoreImage: Self = Self(10005);
    pub const VZErrorInstallationRequiresUpdate: Self = Self(10006);
    pub const VZErrorInstallationFailed: Self = Self(10007);
    pub const VZErrorNetworkBlockDeviceNegotiationFailed: Self = Self(20001);
    pub const VZErrorNetworkBlockDeviceDisconnected: Self = Self(20002);
    pub const VZErrorUSBControllerNotFound: Self = Self(30001);
    pub const VZErrorDeviceAlreadyAttached: Self = Self(30002);
    pub const VZErrorDeviceInitializationFailure: Self = Self(30003);
    pub const VZErrorDeviceNotFound: Self = Self(30004);
}

unsafe impl Encode for VZErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VZErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
