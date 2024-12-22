//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// An object conforming to
    /// `GCButtonElement`represents a momentary switch,
    /// such as a push button.  A button's input only asserts while the user is
    /// interacting with it, and then returns to a preferred state (not pressed, not
    /// touched).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcbuttonelement?language=objc)
    #[cfg(feature = "GCPhysicalInputElement")]
    pub unsafe trait GCButtonElement: GCPhysicalInputElement {
        #[cfg(all(feature = "GCLinearInput", feature = "GCPressedStateInput"))]
        /// Get the input containing the pressed state of the button.
        #[method_id(@__retain_semantics Other pressedInput)]
        unsafe fn pressedInput(
            &self,
        ) -> Retained<AnyObject /* GCPressedStateInput+ GCLinearInput */>;

        #[cfg(feature = "GCTouchedStateInput")]
        /// Get the input containing the touched state of the button.
        ///
        /// Some buttons feature capacitive touch capabilities where the user can touch the
        /// button without pressing it.
        #[method_id(@__retain_semantics Other touchedInput)]
        unsafe fn touchedInput(&self) -> Option<Retained<ProtocolObject<dyn GCTouchedStateInput>>>;
    }

    #[cfg(feature = "GCPhysicalInputElement")]
    unsafe impl ProtocolType for dyn GCButtonElement {}
);
