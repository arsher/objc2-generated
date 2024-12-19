//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextcursorview?language=objc)
    #[cfg(feature = "UIView")]
    pub unsafe trait UITextCursorView: UICoordinateSpace + MainThreadOnly {
        #[method(isBlinking)]
        unsafe fn isBlinking(&self) -> bool;

        #[method(setBlinking:)]
        unsafe fn setBlinking(&self, blinking: bool);

        #[method(resetBlinkAnimation)]
        unsafe fn resetBlinkAnimation(&self);
    }

    #[cfg(feature = "UIView")]
    unsafe impl ProtocolType for dyn UITextCursorView {}
);
