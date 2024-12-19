//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcphysicalinputsourcedirection?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GCPhysicalInputSourceDirection(pub NSUInteger);
bitflags::bitflags! {
    impl GCPhysicalInputSourceDirection: NSUInteger {
        #[doc(alias = "GCPhysicalInputSourceDirectionNotApplicable")]
        const NotApplicable = 0;
        #[doc(alias = "GCPhysicalInputSourceDirectionUp")]
        const Up = 1<<0;
        #[doc(alias = "GCPhysicalInputSourceDirectionRight")]
        const Right = 1<<1;
        #[doc(alias = "GCPhysicalInputSourceDirectionDown")]
        const Down = 1<<2;
        #[doc(alias = "GCPhysicalInputSourceDirectionLeft")]
        const Left = 1<<3;
    }
}

unsafe impl Encode for GCPhysicalInputSourceDirection {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for GCPhysicalInputSourceDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcphysicalinputsource?language=objc)
    pub unsafe trait GCPhysicalInputSource: NSObjectProtocol {
        #[cfg(feature = "GCInputNames")]
        #[method_id(@__retain_semantics Other elementAliases)]
        unsafe fn elementAliases(&self) -> Retained<NSSet<NSString>>;

        #[method_id(@__retain_semantics Other elementLocalizedName)]
        unsafe fn elementLocalizedName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other sfSymbolsName)]
        unsafe fn sfSymbolsName(&self) -> Option<Retained<NSString>>;

        #[method(direction)]
        unsafe fn direction(&self) -> GCPhysicalInputSourceDirection;
    }

    unsafe impl ProtocolType for dyn GCPhysicalInputSource {}
);
