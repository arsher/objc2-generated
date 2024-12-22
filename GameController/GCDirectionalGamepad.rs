//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// The primary directional input surface for the directional gamepad
    ///
    ///
    /// Note: Equivalent to microgamepad.dpad
    ///
    ///
    /// Note: For the 2021 2nd generation Siri Remote, this represents touching anywhere on the entire touch surface - including the inner and outer rings.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcinputdirectionaldpad?language=objc)
    pub static GCInputDirectionalDpad: &'static NSString;
}

extern "C" {
    /// The button corresponding to pressing anywhere on the primary directional input surface for the directional gamepad
    ///
    ///
    /// Note: Equivalent to microgamepad.buttonA
    ///
    ///
    /// Note: For the 2021 2nd generation Siri Remote, this represents pressing anywhere the entire touch surface - including the inner and outer rings.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcinputdirectionaltouchsurfacebutton?language=objc)
    pub static GCInputDirectionalTouchSurfaceButton: &'static NSString;
}

extern "C" {
    /// An optional secondary directional input surface for the directional gamepad. This input is guaranteed to be an 8-way digital dpad with physical Up, Down, Left, Right butttons.
    ///
    ///
    /// Note: For the 2021 2nd generation Siri Remote, this represents pressing on the outer ring of the touch surface.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcinputdirectionalcardinaldpad?language=objc)
    pub static GCInputDirectionalCardinalDpad: &'static NSString;
}

extern "C" {
    /// An optional button for the directional gamepad. This input represents the center button of the cardinal dpad.
    ///
    ///
    /// Note: For the 2021 2nd generation Siri Remote, this represents pressing anywhere on the inner ring of the touch surface.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcinputdirectionalcenterbutton?language=objc)
    pub static GCInputDirectionalCenterButton: &'static NSString;
}

extern_class!(
    /// Directional Gamepad profile.
    ///
    /// All controller profiles provide a base level of information about the controller they belong to. A directional gamepad
    /// features a subset of the possible inputs on a micro gamepad. It guarantees:
    /// - The gamepad does not support motion, meaning
    /// - -[GCController motion] is always nil
    /// - -[GCDirectionalGamepad allowsRotation] is always NO
    ///
    /// Additionally, the gamepad may have a digital or analog dpad.
    /// - -[GCDirectionalGamepad dpad].analog may be YES or NO
    /// - If -[GCDirectionalGamepad dpad].analog is NO, then -[GCDirectionalGamepad reportsAbsoluteDpadValues] is always YES
    ///
    /// A profile maps the hardware notion of a controller into a logical controller. One that a developer can design for
    /// and depend on, no matter the underlying hardware. If your game supports GCMicroGamepad, but does not need
    /// the motion and analog dpad functionality of GCMicroGamepad, be sure to add Directional Gamepad to your project's
    /// supported Game Controller capabilities.
    ///
    ///
    /// See: GCMicroGamepad
    ///
    ///
    /// Note: If you want to use the additional functionality of GCDirectionalGamepad, you should set GCSupportsMultipleMicroGamepads to YES and handle microgamepad connections separately.
    ///
    ///
    /// Note: This profile represents the 2021 2nd generation Siri Remote. Make sure you set GCSupportsMultipleMicroGamepads to YES to properly support the remote.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdirectionalgamepad?language=objc)
    #[unsafe(super(GCMicroGamepad, GCPhysicalInputProfile, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "GCMicroGamepad", feature = "GCPhysicalInputProfile"))]
    pub struct GCDirectionalGamepad;
);

#[cfg(all(feature = "GCMicroGamepad", feature = "GCPhysicalInputProfile"))]
unsafe impl NSObjectProtocol for GCDirectionalGamepad {}

extern_methods!(
    #[cfg(all(feature = "GCMicroGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCDirectionalGamepad {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "GCMicroGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCDirectionalGamepad {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
