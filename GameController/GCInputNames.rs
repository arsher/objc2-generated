//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcphysicalinputelementname?language=objc)
    pub unsafe trait GCPhysicalInputElementName {}

    unsafe impl ProtocolType for dyn GCPhysicalInputElementName {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcbuttonelementname?language=objc)
    pub unsafe trait GCButtonElementName: GCPhysicalInputElementName {}

    unsafe impl ProtocolType for dyn GCButtonElementName {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcaxiselementname?language=objc)
    pub unsafe trait GCAxisElementName: GCPhysicalInputElementName {}

    unsafe impl ProtocolType for dyn GCAxisElementName {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcswitchelementname?language=objc)
    pub unsafe trait GCSwitchElementName: GCPhysicalInputElementName {}

    unsafe impl ProtocolType for dyn GCSwitchElementName {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdirectionpadelementname?language=objc)
    pub unsafe trait GCDirectionPadElementName: GCPhysicalInputElementName {}

    unsafe impl ProtocolType for dyn GCDirectionPadElementName {}
);

extern "C" {
    /// Identifies the button element located at the top-left/right of a gamepad,
    /// between the left/right shoulder button and the gamepad's horizontal center.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcinputleftbumper?language=objc)
    pub static GCInputLeftBumper: Option<&'static GCInputButtonName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcinputrightbumper?language=objc)
    pub static GCInputRightBumper: Option<&'static GCInputButtonName>;
}

#[inline]
pub unsafe extern "C-unwind" fn GCInputBackLeftButton(
    position: NSInteger,
) -> Option<Retained<GCInputButtonName>> {
    extern "C-unwind" {
        fn GCInputBackLeftButton(position: NSInteger) -> *mut GCInputButtonName;
    }
    let ret = unsafe { GCInputBackLeftButton(position) };
    unsafe { Retained::retain_autoreleased(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn GCInputBackRightButton(
    position: NSInteger,
) -> Option<Retained<GCInputButtonName>> {
    extern "C-unwind" {
        fn GCInputBackRightButton(position: NSInteger) -> *mut GCInputButtonName;
    }
    let ret = unsafe { GCInputBackRightButton(position) };
    unsafe { Retained::retain_autoreleased(ret) }
}
