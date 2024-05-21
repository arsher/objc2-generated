//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCAcceleration {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}

unsafe impl Encode for GCAcceleration {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GCAcceleration {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCRotationRate {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}

unsafe impl Encode for GCRotationRate {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GCRotationRate {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCEulerAngles {
    pub pitch: c_double,
    pub yaw: c_double,
    pub roll: c_double,
}

unsafe impl Encode for GCEulerAngles {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GCEulerAngles {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCQuaternion {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

unsafe impl Encode for GCQuaternion {
    const ENCODING: Encoding = Encoding::Struct(
        "GCQuaternion",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GCQuaternion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "block2")]
pub type GCMotionValueChangedHandler = *mut block2::Block<dyn Fn(NonNull<GCMotion>)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCMotion;

    unsafe impl ClassType for GCMotion {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GCMotion {}

extern_methods!(
    unsafe impl GCMotion {
        #[cfg(feature = "GCController")]
        #[method_id(@__retain_semantics Other controller)]
        pub unsafe fn controller(&self) -> Option<Retained<GCController>>;

        #[cfg(feature = "block2")]
        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCMotionValueChangedHandler;

        #[cfg(feature = "block2")]
        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCMotionValueChangedHandler,
        );

        #[method(sensorsRequireManualActivation)]
        pub unsafe fn sensorsRequireManualActivation(&self) -> bool;

        #[method(sensorsActive)]
        pub unsafe fn sensorsActive(&self) -> bool;

        #[method(setSensorsActive:)]
        pub unsafe fn setSensorsActive(&self, sensors_active: bool);

        #[method(hasGravityAndUserAcceleration)]
        pub unsafe fn hasGravityAndUserAcceleration(&self) -> bool;

        #[method(gravity)]
        pub unsafe fn gravity(&self) -> GCAcceleration;

        #[method(userAcceleration)]
        pub unsafe fn userAcceleration(&self) -> GCAcceleration;

        #[method(acceleration)]
        pub unsafe fn acceleration(&self) -> GCAcceleration;

        #[deprecated = "hasAttitudeAndRotationRate has been deprecated, use -hasAttitude and -hasRotationRate instead"]
        #[method(hasAttitudeAndRotationRate)]
        pub unsafe fn hasAttitudeAndRotationRate(&self) -> bool;

        #[method(hasAttitude)]
        pub unsafe fn hasAttitude(&self) -> bool;

        #[method(hasRotationRate)]
        pub unsafe fn hasRotationRate(&self) -> bool;

        #[method(attitude)]
        pub unsafe fn attitude(&self) -> GCQuaternion;

        #[method(rotationRate)]
        pub unsafe fn rotationRate(&self) -> GCRotationRate;

        #[method(setGravity:)]
        pub unsafe fn setGravity(&self, gravity: GCAcceleration);

        #[method(setUserAcceleration:)]
        pub unsafe fn setUserAcceleration(&self, user_acceleration: GCAcceleration);

        #[method(setAcceleration:)]
        pub unsafe fn setAcceleration(&self, acceleration: GCAcceleration);

        #[method(setAttitude:)]
        pub unsafe fn setAttitude(&self, attitude: GCQuaternion);

        #[method(setRotationRate:)]
        pub unsafe fn setRotationRate(&self, rotation_rate: GCRotationRate);

        #[method(setStateFromMotion:)]
        pub unsafe fn setStateFromMotion(&self, motion: &GCMotion);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCMotion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
