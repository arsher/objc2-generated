//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCColor")]
    pub struct GCColor;

    #[cfg(feature = "GameController_GCColor")]
    unsafe impl ClassType for GCColor {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameController_GCColor")]
unsafe impl NSCoding for GCColor {}

#[cfg(feature = "GameController_GCColor")]
unsafe impl NSObjectProtocol for GCColor {}

#[cfg(feature = "GameController_GCColor")]
unsafe impl NSSecureCoding for GCColor {}

extern_methods!(
    #[cfg(feature = "GameController_GCColor")]
    unsafe impl GCColor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithRed:green:blue:)]
        pub unsafe fn initWithRed_green_blue(
            this: Option<Allocated<Self>>,
            red: c_float,
            green: c_float,
            blue: c_float,
        ) -> Id<Self, Shared>;

        #[method(red)]
        pub unsafe fn red(&self) -> c_float;

        #[method(green)]
        pub unsafe fn green(&self) -> c_float;

        #[method(blue)]
        pub unsafe fn blue(&self) -> c_float;
    }
);
