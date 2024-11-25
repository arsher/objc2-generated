//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    pub static NSFoundationVersionNumber: c_double;
}

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSExceptionName = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSRunLoopMode = NSString;

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSStringFromSelector(a_selector: Sel) -> NonNull<NSString>;
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSSelectorFromString(a_selector_name: &NSString) -> Sel;
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSStringFromClass(a_class: &AnyClass) -> NonNull<NSString>;
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSClassFromString(a_class_name: &NSString) -> Option<&'static AnyClass>;
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSStringFromProtocol(proto: &AnyProtocol) -> NonNull<NSString>;
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSProtocolFromString(namestr: &NSString) -> *mut AnyProtocol;
}

extern "C-unwind" {
    pub fn NSGetSizeAndAlignment(
        type_ptr: NonNull<c_char>,
        sizep: *mut NSUInteger,
        alignp: *mut NSUInteger,
    ) -> NonNull<c_char>;
}

#[cfg(feature = "block2")]
pub type NSComparator =
    *mut block2::Block<dyn Fn(NonNull<AnyObject>, NonNull<AnyObject>) -> NSComparisonResult>;

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

pub static NSNotFound: NSInteger = NSIntegerMax as _;
