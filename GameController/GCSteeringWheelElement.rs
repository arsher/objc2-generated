//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcsteeringwheelelement?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCSteeringWheelElement;
);

#[cfg(all(feature = "GCAxisElement", feature = "GCPhysicalInputElement"))]
unsafe impl GCAxisElement for GCSteeringWheelElement {}

#[cfg(feature = "GCPhysicalInputElement")]
unsafe impl GCPhysicalInputElement for GCSteeringWheelElement {}

unsafe impl NSObjectProtocol for GCSteeringWheelElement {}

extern_methods!(
    unsafe impl GCSteeringWheelElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(maximumDegreesOfRotation)]
        pub unsafe fn maximumDegreesOfRotation(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCSteeringWheelElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
