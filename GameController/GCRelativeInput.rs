//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcrelativeinput?language=objc)
    pub unsafe trait GCRelativeInput: NSObjectProtocol {
        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        #[method(deltaDidChangeHandler)]
        unsafe fn deltaDidChangeHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(
                NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                NonNull<ProtocolObject<dyn GCRelativeInput>>,
                c_float,
            ),
        >;

        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        #[method(setDeltaDidChangeHandler:)]
        unsafe fn setDeltaDidChangeHandler(
            &self,
            delta_did_change_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                        NonNull<ProtocolObject<dyn GCRelativeInput>>,
                        c_float,
                    ),
                >,
            >,
        );

        #[method(delta)]
        unsafe fn delta(&self) -> c_float;

        #[method(isAnalog)]
        unsafe fn isAnalog(&self) -> bool;

        #[method(lastDeltaTimestamp)]
        unsafe fn lastDeltaTimestamp(&self) -> NSTimeInterval;

        #[method(lastDeltaLatency)]
        unsafe fn lastDeltaLatency(&self) -> NSTimeInterval;

        #[cfg(feature = "GCPhysicalInputSource")]
        #[method_id(@__retain_semantics Other sources)]
        unsafe fn sources(&self) -> Retained<NSSet<ProtocolObject<dyn GCPhysicalInputSource>>>;
    }

    unsafe impl ProtocolType for dyn GCRelativeInput {}
);
