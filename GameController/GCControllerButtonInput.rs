//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

pub type GCControllerButtonValueChangedHandler =
    *mut Block<(NonNull<GCControllerButtonInput>, c_float, Bool), ()>;

pub type GCControllerButtonTouchedChangedHandler =
    *mut Block<(NonNull<GCControllerButtonInput>, c_float, Bool, Bool), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCControllerButtonInput")]
    pub struct GCControllerButtonInput;

    #[cfg(feature = "GameController_GCControllerButtonInput")]
    unsafe impl ClassType for GCControllerButtonInput {
        #[inherits(NSObject)]
        type Super = GCControllerElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCControllerButtonInput")]
unsafe impl NSObjectProtocol for GCControllerButtonInput {}

extern_methods!(
    #[cfg(feature = "GameController_GCControllerButtonInput")]
    unsafe impl GCControllerButtonInput {
        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCControllerButtonValueChangedHandler;

        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCControllerButtonValueChangedHandler,
        );

        #[method(pressedChangedHandler)]
        pub unsafe fn pressedChangedHandler(&self) -> GCControllerButtonValueChangedHandler;

        #[method(setPressedChangedHandler:)]
        pub unsafe fn setPressedChangedHandler(
            &self,
            pressed_changed_handler: GCControllerButtonValueChangedHandler,
        );

        #[method(touchedChangedHandler)]
        pub unsafe fn touchedChangedHandler(&self) -> GCControllerButtonTouchedChangedHandler;

        #[method(setTouchedChangedHandler:)]
        pub unsafe fn setTouchedChangedHandler(
            &self,
            touched_changed_handler: GCControllerButtonTouchedChangedHandler,
        );

        #[method(value)]
        pub unsafe fn value(&self) -> c_float;

        #[method(isPressed)]
        pub unsafe fn isPressed(&self) -> bool;

        #[method(isTouched)]
        pub unsafe fn isTouched(&self) -> bool;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_float);
    }
);
