//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfoundationversionnumber?language=objc)
    pub static NSFoundationVersionNumber: c_double;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsexceptionname?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSExceptionName = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrunloopmode?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSRunLoopMode = NSString;

#[cfg(feature = "NSString")]
#[inline]
pub unsafe extern "C-unwind" fn NSStringFromSelector(a_selector: Sel) -> Retained<NSString> {
    extern "C-unwind" {
        fn NSStringFromSelector(a_selector: Sel) -> NonNull<NSString>;
    }
    let ret = unsafe { NSStringFromSelector(a_selector) };
    unsafe { Retained::retain_autoreleased(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSSelectorFromString(a_selector_name: &NSString) -> Sel;
}

#[cfg(feature = "NSString")]
#[inline]
pub unsafe extern "C-unwind" fn NSStringFromClass(a_class: &AnyClass) -> Retained<NSString> {
    extern "C-unwind" {
        fn NSStringFromClass(a_class: &AnyClass) -> NonNull<NSString>;
    }
    let ret = unsafe { NSStringFromClass(a_class) };
    unsafe { Retained::retain_autoreleased(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSClassFromString(a_class_name: &NSString) -> Option<&'static AnyClass>;
}

#[cfg(feature = "NSString")]
#[inline]
pub unsafe extern "C-unwind" fn NSStringFromProtocol(proto: &AnyProtocol) -> Retained<NSString> {
    extern "C-unwind" {
        fn NSStringFromProtocol(proto: &AnyProtocol) -> NonNull<NSString>;
    }
    let ret = unsafe { NSStringFromProtocol(proto) };
    unsafe { Retained::retain_autoreleased(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

#[cfg(feature = "NSString")]
#[inline]
pub unsafe extern "C-unwind" fn NSProtocolFromString(
    namestr: &NSString,
) -> Option<Retained<AnyProtocol>> {
    extern "C-unwind" {
        fn NSProtocolFromString(namestr: &NSString) -> *mut AnyProtocol;
    }
    let ret = unsafe { NSProtocolFromString(namestr) };
    unsafe { Retained::retain_autoreleased(ret) }
}

extern "C-unwind" {
    pub fn NSGetSizeAndAlignment(
        type_ptr: NonNull<c_char>,
        sizep: *mut NSUInteger,
        alignp: *mut NSUInteger,
    ) -> NonNull<c_char>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscomparator?language=objc)
#[cfg(feature = "block2")]
pub type NSComparator =
    *mut block2::Block<dyn Fn(NonNull<AnyObject>, NonNull<AnyObject>) -> NSComparisonResult>;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsenumerationoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEnumerationOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSEnumerationOptions: NSUInteger {
        const NSEnumerationConcurrent = 1<<0;
        const NSEnumerationReverse = 1<<1;
    }
}

unsafe impl Encode for NSEnumerationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSEnumerationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nssortoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSortOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSSortOptions: NSUInteger {
        const NSSortConcurrent = 1<<0;
        const NSSortStable = 1<<4;
    }
}

unsafe impl Encode for NSSortOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSSortOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsqualityofservice?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSQualityOfService(pub NSInteger);
impl NSQualityOfService {
    #[doc(alias = "NSQualityOfServiceUserInteractive")]
    pub const UserInteractive: Self = Self(0x21);
    #[doc(alias = "NSQualityOfServiceUserInitiated")]
    pub const UserInitiated: Self = Self(0x19);
    #[doc(alias = "NSQualityOfServiceUtility")]
    pub const Utility: Self = Self(0x11);
    #[doc(alias = "NSQualityOfServiceBackground")]
    pub const Background: Self = Self(0x09);
    #[doc(alias = "NSQualityOfServiceDefault")]
    pub const Default: Self = Self(-1);
}

unsafe impl Encode for NSQualityOfService {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSQualityOfService {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnotfound?language=objc)
pub static NSNotFound: NSInteger = NSIntegerMax as _;
