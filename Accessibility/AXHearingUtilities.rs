//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/accessibility/axhearingdeviceear?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AXHearingDeviceEar(pub NSUInteger);
bitflags::bitflags! {
    impl AXHearingDeviceEar: NSUInteger {
        #[doc(alias = "AXHearingDeviceEarNone")]
        const None = 0;
        #[doc(alias = "AXHearingDeviceEarLeft")]
        const Left = 1<<1;
        #[doc(alias = "AXHearingDeviceEarRight")]
        const Right = 1<<2;
        #[doc(alias = "AXHearingDeviceEarBoth")]
        const Both = AXHearingDeviceEar::Left.0|AXHearingDeviceEar::Right.0;
    }
}

unsafe impl Encode for AXHearingDeviceEar {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AXHearingDeviceEar {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    pub fn AXMFiHearingDeviceStreamingEar() -> AXHearingDeviceEar;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accessibility/axmfihearingdevicestreamingeardidchangenotification?language=objc)
    pub static AXMFiHearingDeviceStreamingEarDidChangeNotification: &'static NSNotificationName;
}

extern "C-unwind" {
    pub fn AXSupportsBidirectionalAXMFiHearingDeviceStreaming() -> Bool;
}

extern "C-unwind" {
    pub fn AXMFiHearingDevicePairedUUIDs() -> NonNull<NSArray<NSUUID>>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accessibility/axmfihearingdevicepaireduuidsdidchangenotification?language=objc)
    pub static AXMFiHearingDevicePairedUUIDsDidChangeNotification: &'static NSNotificationName;
}
