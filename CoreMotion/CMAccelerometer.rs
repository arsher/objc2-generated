//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coremotion/cmacceleration?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMAcceleration {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}

unsafe impl Encode for CMAcceleration {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CMAcceleration {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coremotion/cmaccelerometerdata?language=objc)
    #[unsafe(super(CMLogItem, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CMLogItem")]
    pub struct CMAccelerometerData;
);

#[cfg(feature = "CMLogItem")]
unsafe impl NSCoding for CMAccelerometerData {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSCopying for CMAccelerometerData {}

#[cfg(feature = "CMLogItem")]
unsafe impl CopyingHelper for CMAccelerometerData {
    type Result = Self;
}

#[cfg(feature = "CMLogItem")]
unsafe impl NSObjectProtocol for CMAccelerometerData {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSSecureCoding for CMAccelerometerData {}

extern_methods!(
    #[cfg(feature = "CMLogItem")]
    unsafe impl CMAccelerometerData {
        #[method(acceleration)]
        pub unsafe fn acceleration(&self) -> CMAcceleration;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CMLogItem")]
    unsafe impl CMAccelerometerData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
