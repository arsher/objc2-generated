//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-metal")]
#[cfg(not(target_os = "watchos"))]
use objc2_metal::*;

use crate::*;

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "objc2-metal"))]
    #[cfg(not(target_os = "watchos"))]
    pub fn CGDirectDisplayCopyCurrentMetalDevice(
        display: CGDirectDisplayID,
    ) -> *mut ProtocolObject<dyn MTLDevice>;
}