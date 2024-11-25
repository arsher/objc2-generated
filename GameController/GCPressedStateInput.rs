//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcpressedstateinput?language=objc)
    pub unsafe trait GCPressedStateInput: NSObjectProtocol {
        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        #[method(pressedDidChangeHandler)]
        unsafe fn pressedDidChangeHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(
                NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                NonNull<ProtocolObject<dyn GCPressedStateInput>>,
                Bool,
            ),
        >;

        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        #[method(setPressedDidChangeHandler:)]
        unsafe fn setPressedDidChangeHandler(
            &self,
            pressed_did_change_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                        NonNull<ProtocolObject<dyn GCPressedStateInput>>,
                        Bool,
                    ),
                >,
            >,
        );

        #[method(isPressed)]
        unsafe fn isPressed(&self) -> bool;

        #[method(lastPressedStateTimestamp)]
        unsafe fn lastPressedStateTimestamp(&self) -> NSTimeInterval;

        #[method(lastPressedStateLatency)]
        unsafe fn lastPressedStateLatency(&self) -> NSTimeInterval;

        #[cfg(feature = "GCPhysicalInputSource")]
        #[method_id(@__retain_semantics Other sources)]
        unsafe fn sources(&self) -> Retained<NSSet<ProtocolObject<dyn GCPhysicalInputSource>>>;
    }

    unsafe impl ProtocolType for dyn GCPressedStateInput {}
);
