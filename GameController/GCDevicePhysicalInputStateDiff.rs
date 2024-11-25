//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevicephysicalinputelementchange?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GCDevicePhysicalInputElementChange(pub NSInteger);
impl GCDevicePhysicalInputElementChange {
    pub const GCDevicePhysicalInputElementUnknownChange: Self = Self(-1);
    pub const GCDevicePhysicalInputElementNoChange: Self = Self(0);
    #[doc(alias = "GCDevicePhysicalInputElementChanged")]
    pub const d: Self = Self(1);
}

unsafe impl Encode for GCDevicePhysicalInputElementChange {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GCDevicePhysicalInputElementChange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevicephysicalinputstatediff?language=objc)
    pub unsafe trait GCDevicePhysicalInputStateDiff: NSObjectProtocol {
        #[cfg(feature = "GCPhysicalInputElement")]
        #[method(changeForElement:)]
        unsafe fn changeForElement(
            &self,
            element: &ProtocolObject<dyn GCPhysicalInputElement>,
        ) -> GCDevicePhysicalInputElementChange;

        #[cfg(feature = "GCPhysicalInputElement")]
        #[method_id(@__retain_semantics Other changedElements)]
        unsafe fn changedElements(
            &self,
        ) -> Option<Retained<NSEnumerator<ProtocolObject<dyn GCPhysicalInputElement>>>>;
    }

    unsafe impl ProtocolType for dyn GCDevicePhysicalInputStateDiff {}
);
