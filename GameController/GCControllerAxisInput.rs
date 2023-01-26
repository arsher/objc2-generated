//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

pub type GCControllerAxisValueChangedHandler =
    *mut Block<(NonNull<GCControllerAxisInput>, c_float), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCControllerAxisInput")]
    pub struct GCControllerAxisInput;

    #[cfg(feature = "GameController_GCControllerAxisInput")]
    unsafe impl ClassType for GCControllerAxisInput {
        #[inherits(NSObject)]
        type Super = GCControllerElement;
    }
);

#[cfg(feature = "GameController_GCControllerAxisInput")]
unsafe impl NSObjectProtocol for GCControllerAxisInput {}

extern_methods!(
    #[cfg(feature = "GameController_GCControllerAxisInput")]
    unsafe impl GCControllerAxisInput {
        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCControllerAxisValueChangedHandler;

        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCControllerAxisValueChangedHandler,
        );

        #[method(value)]
        pub unsafe fn value(&self) -> c_float;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_float);
    }
);
