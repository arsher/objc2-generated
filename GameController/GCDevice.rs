//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevice?language=objc)
    pub unsafe trait GCDevice: NSObjectProtocol {
        #[method_id(@__retain_semantics Other vendorName)]
        unsafe fn vendorName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other productCategory)]
        unsafe fn productCategory(&self) -> Retained<NSString>;

        #[cfg(feature = "GCPhysicalInputProfile")]
        #[deprecated = "Use the physicalInputProfile property on GCController instead.  For GCKeyboard, use the keyboardInput property.  For GCMouse, use the mouseInput property."]
        #[method_id(@__retain_semantics Other physicalInputProfile)]
        unsafe fn physicalInputProfile(&self) -> Retained<GCPhysicalInputProfile>;
    }

    unsafe impl ProtocolType for dyn GCDevice {}
);
