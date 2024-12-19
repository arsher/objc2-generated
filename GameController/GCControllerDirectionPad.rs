//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gccontrollerdirectionpadvaluechangedhandler?language=objc)
#[cfg(all(feature = "GCControllerElement", feature = "block2"))]
pub type GCControllerDirectionPadValueChangedHandler =
    *mut block2::Block<dyn Fn(NonNull<GCControllerDirectionPad>, c_float, c_float)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gccontrollerdirectionpad?language=objc)
    #[unsafe(super(GCControllerElement, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GCControllerElement")]
    pub struct GCControllerDirectionPad;
);

#[cfg(feature = "GCControllerElement")]
unsafe impl NSObjectProtocol for GCControllerDirectionPad {}

extern_methods!(
    #[cfg(feature = "GCControllerElement")]
    unsafe impl GCControllerDirectionPad {
        #[cfg(feature = "block2")]
        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCControllerDirectionPadValueChangedHandler;

        #[cfg(feature = "block2")]
        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCControllerDirectionPadValueChangedHandler,
        );

        #[cfg(feature = "GCControllerAxisInput")]
        #[method_id(@__retain_semantics Other xAxis)]
        pub unsafe fn xAxis(&self) -> Retained<GCControllerAxisInput>;

        #[cfg(feature = "GCControllerAxisInput")]
        #[method_id(@__retain_semantics Other yAxis)]
        pub unsafe fn yAxis(&self) -> Retained<GCControllerAxisInput>;

        #[cfg(feature = "GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other up)]
        pub unsafe fn up(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(feature = "GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other down)]
        pub unsafe fn down(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(feature = "GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other left)]
        pub unsafe fn left(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(feature = "GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other right)]
        pub unsafe fn right(&self) -> Retained<GCControllerButtonInput>;

        #[method(setValueForXAxis:yAxis:)]
        pub unsafe fn setValueForXAxis_yAxis(&self, x_axis: c_float, y_axis: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GCControllerElement")]
    unsafe impl GCControllerDirectionPad {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
