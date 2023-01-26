//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCDualSenseGamepad")]
    pub struct GCDualSenseGamepad;

    #[cfg(feature = "GameController_GCDualSenseGamepad")]
    unsafe impl ClassType for GCDualSenseGamepad {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCExtendedGamepad;
    }
);

#[cfg(feature = "GameController_GCDualSenseGamepad")]
unsafe impl NSObjectProtocol for GCDualSenseGamepad {}

extern_methods!(
    #[cfg(feature = "GameController_GCDualSenseGamepad")]
    unsafe impl GCDualSenseGamepad {
        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other touchpadButton)]
        pub unsafe fn touchpadButton(&self) -> Id<GCControllerButtonInput, Shared>;

        #[cfg(feature = "GameController_GCControllerDirectionPad")]
        #[method_id(@__retain_semantics Other touchpadPrimary)]
        pub unsafe fn touchpadPrimary(&self) -> Id<GCControllerDirectionPad, Shared>;

        #[cfg(feature = "GameController_GCControllerDirectionPad")]
        #[method_id(@__retain_semantics Other touchpadSecondary)]
        pub unsafe fn touchpadSecondary(&self) -> Id<GCControllerDirectionPad, Shared>;

        #[cfg(feature = "GameController_GCDualSenseAdaptiveTrigger")]
        #[method_id(@__retain_semantics Other leftTrigger)]
        pub unsafe fn leftTrigger(&self) -> Id<GCDualSenseAdaptiveTrigger, Shared>;

        #[cfg(feature = "GameController_GCDualSenseAdaptiveTrigger")]
        #[method_id(@__retain_semantics Other rightTrigger)]
        pub unsafe fn rightTrigger(&self) -> Id<GCDualSenseAdaptiveTrigger, Shared>;
    }
);
