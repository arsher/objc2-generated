//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsshadow?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSShadow;
);

unsafe impl NSCoding for NSShadow {}

unsafe impl NSCopying for NSShadow {}

unsafe impl CopyingHelper for NSShadow {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSShadow {}

unsafe impl NSSecureCoding for NSShadow {}

extern_methods!(
    unsafe impl NSShadow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(shadowOffset)]
        pub unsafe fn shadowOffset(&self) -> NSSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`shadowOffset`][Self::shadowOffset].
        #[method(setShadowOffset:)]
        pub unsafe fn setShadowOffset(&self, shadow_offset: NSSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(shadowBlurRadius)]
        pub unsafe fn shadowBlurRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`shadowBlurRadius`][Self::shadowBlurRadius].
        #[method(setShadowBlurRadius:)]
        pub unsafe fn setShadowBlurRadius(&self, shadow_blur_radius: CGFloat);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other shadowColor)]
        pub unsafe fn shadowColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        /// Setter for [`shadowColor`][Self::shadowColor].
        #[method(setShadowColor:)]
        pub unsafe fn setShadowColor(&self, shadow_color: Option<&NSColor>);

        #[method(set)]
        pub unsafe fn set(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSShadow {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
