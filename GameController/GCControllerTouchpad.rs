//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GCTouchState {
        GCTouchStateUp = 0,
        GCTouchStateDown = 1,
        GCTouchStateMoving = 2,
    }
);

pub type GCControllerTouchpadHandler = *mut Block<
    (
        NonNull<GCControllerTouchpad>,
        c_float,
        c_float,
        c_float,
        Bool,
    ),
    (),
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCControllerTouchpad")]
    pub struct GCControllerTouchpad;

    #[cfg(feature = "GameController_GCControllerTouchpad")]
    unsafe impl ClassType for GCControllerTouchpad {
        #[inherits(NSObject)]
        type Super = GCControllerElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCControllerTouchpad")]
unsafe impl NSObjectProtocol for GCControllerTouchpad {}

extern_methods!(
    #[cfg(feature = "GameController_GCControllerTouchpad")]
    unsafe impl GCControllerTouchpad {
        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other button)]
        pub unsafe fn button(&self) -> Id<GCControllerButtonInput>;

        #[method(touchDown)]
        pub unsafe fn touchDown(&self) -> GCControllerTouchpadHandler;

        #[method(setTouchDown:)]
        pub unsafe fn setTouchDown(&self, touch_down: GCControllerTouchpadHandler);

        #[method(touchMoved)]
        pub unsafe fn touchMoved(&self) -> GCControllerTouchpadHandler;

        #[method(setTouchMoved:)]
        pub unsafe fn setTouchMoved(&self, touch_moved: GCControllerTouchpadHandler);

        #[method(touchUp)]
        pub unsafe fn touchUp(&self) -> GCControllerTouchpadHandler;

        #[method(setTouchUp:)]
        pub unsafe fn setTouchUp(&self, touch_up: GCControllerTouchpadHandler);

        #[cfg(feature = "GameController_GCControllerDirectionPad")]
        #[method_id(@__retain_semantics Other touchSurface)]
        pub unsafe fn touchSurface(&self) -> Id<GCControllerDirectionPad>;

        #[method(touchState)]
        pub unsafe fn touchState(&self) -> GCTouchState;

        #[method(reportsAbsoluteTouchSurfaceValues)]
        pub unsafe fn reportsAbsoluteTouchSurfaceValues(&self) -> bool;

        #[method(setReportsAbsoluteTouchSurfaceValues:)]
        pub unsafe fn setReportsAbsoluteTouchSurfaceValues(
            &self,
            reports_absolute_touch_surface_values: bool,
        );

        #[method(setValueForXAxis:yAxis:touchDown:buttonValue:)]
        pub unsafe fn setValueForXAxis_yAxis_touchDown_buttonValue(
            &self,
            x_axis: c_float,
            y_axis: c_float,
            touch_down: bool,
            button_value: c_float,
        );
    }
);
