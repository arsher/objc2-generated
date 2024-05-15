//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMRotationRate {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}

unsafe impl Encode for CMRotationRate {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CMRotationRate {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CMLogItem")]
    pub struct CMGyroData;

    #[cfg(feature = "CMLogItem")]
    unsafe impl ClassType for CMGyroData {
        #[inherits(NSObject)]
        type Super = CMLogItem;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CMLogItem")]
unsafe impl NSCoding for CMGyroData {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSCopying for CMGyroData {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSObjectProtocol for CMGyroData {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSSecureCoding for CMGyroData {}

extern_methods!(
    #[cfg(feature = "CMLogItem")]
    unsafe impl CMGyroData {
        #[method(rotationRate)]
        pub unsafe fn rotationRate(&self) -> CMRotationRate;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CMLogItem")]
    unsafe impl CMGyroData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);