//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait GCTouchedStateInput: NSObjectProtocol {
        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        #[method(touchedDidChangeHandler)]
        unsafe fn touchedDidChangeHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(
                NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                NonNull<ProtocolObject<dyn GCTouchedStateInput>>,
                Bool,
            ),
        >;

        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        #[method(setTouchedDidChangeHandler:)]
        unsafe fn setTouchedDidChangeHandler(
            &self,
            touched_did_change_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                        NonNull<ProtocolObject<dyn GCTouchedStateInput>>,
                        Bool,
                    ),
                >,
            >,
        );

        #[method(isTouched)]
        unsafe fn isTouched(&self) -> bool;

        #[method(lastTouchedStateTimestamp)]
        unsafe fn lastTouchedStateTimestamp(&self) -> NSTimeInterval;

        #[method(lastTouchedStateLatency)]
        unsafe fn lastTouchedStateLatency(&self) -> NSTimeInterval;

        #[cfg(feature = "GCPhysicalInputSource")]
        #[method_id(@__retain_semantics Other sources)]
        unsafe fn sources(&self) -> Retained<NSSet<ProtocolObject<dyn GCPhysicalInputSource>>>;
    }

    unsafe impl ProtocolType for dyn GCTouchedStateInput {}
);
