//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-graphics")]
    pub fn AXNameFromColor(color: CGColorRef) -> NonNull<NSString>;
}
