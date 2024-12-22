//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A cursor is a Direction Pad that has its axis extended from [-1; 1] to [width; height] range
    /// Up, down, left, right allows to use mouse to simulate DirectionaPad or Thumbstick since values are normalized for these elements
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevicecursor?language=objc)
    #[unsafe(super(GCControllerDirectionPad, GCControllerElement, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
    pub struct GCDeviceCursor;
);

#[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
unsafe impl NSObjectProtocol for GCDeviceCursor {}

extern_methods!(
    #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
    unsafe impl GCDeviceCursor {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
    unsafe impl GCDeviceCursor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
