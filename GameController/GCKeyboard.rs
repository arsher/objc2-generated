//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_static!(GCKeyboardDidConnectNotification: &'static NSString);

extern_static!(GCKeyboardDidDisconnectNotification: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCKeyboard")]
    pub struct GCKeyboard;

    #[cfg(feature = "GameController_GCKeyboard")]
    unsafe impl ClassType for GCKeyboard {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameController_GCKeyboard")]
unsafe impl GCDevice for GCKeyboard {}

#[cfg(feature = "GameController_GCKeyboard")]
unsafe impl NSObjectProtocol for GCKeyboard {}

extern_methods!(
    #[cfg(feature = "GameController_GCKeyboard")]
    unsafe impl GCKeyboard {
        #[cfg(feature = "GameController_GCKeyboardInput")]
        #[method_id(@__retain_semantics Other keyboardInput)]
        pub unsafe fn keyboardInput(&self) -> Option<Id<GCKeyboardInput, Shared>>;

        #[method_id(@__retain_semantics Other coalescedKeyboard)]
        pub unsafe fn coalescedKeyboard() -> Option<Id<GCKeyboard, Shared>>;
    }
);
