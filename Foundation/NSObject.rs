//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscoding?language=objc)
    pub unsafe trait NSCoding {
        #[cfg(feature = "NSCoder")]
        #[method(encodeWithCoder:)]
        unsafe fn encodeWithCoder(&self, coder: &NSCoder);

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Retained<Self>>;
    }

    unsafe impl ProtocolType for dyn NSCoding {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nssecurecoding?language=objc)
    pub unsafe trait NSSecureCoding: NSCoding {
        #[method(supportsSecureCoding)]
        unsafe fn supportsSecureCoding() -> bool;
    }

    unsafe impl ProtocolType for dyn NSSecureCoding {}
);

extern_category!(
    /// Category "NSCoderMethods" on [`NSObject`].
    #[doc(alias = "NSCoderMethods")]
    pub unsafe trait NSObjectNSCoderMethods {
        #[method(version)]
        unsafe fn version() -> NSInteger;

        #[method(setVersion:)]
        unsafe fn setVersion(a_version: NSInteger);

        #[method(classForCoder)]
        unsafe fn classForCoder(&self) -> &'static AnyClass;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Other replacementObjectForCoder:)]
        unsafe fn replacementObjectForCoder(&self, coder: &NSCoder) -> Option<Retained<AnyObject>>;
    }

    unsafe impl NSObjectNSCoderMethods for NSObject {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdiscardablecontent?language=objc)
    pub unsafe trait NSDiscardableContent {
        #[method(beginContentAccess)]
        unsafe fn beginContentAccess(&self) -> bool;

        #[method(endContentAccess)]
        unsafe fn endContentAccess(&self);

        #[method(discardContentIfPossible)]
        unsafe fn discardContentIfPossible(&self);

        #[method(isContentDiscarded)]
        unsafe fn isContentDiscarded(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn NSDiscardableContent {}
);

extern_category!(
    /// Category "NSDiscardableContentProxy" on [`NSObject`].
    #[doc(alias = "NSDiscardableContentProxy")]
    pub unsafe trait NSObjectNSDiscardableContentProxy {
        #[method_id(@__retain_semantics Other autoContentAccessingProxy)]
        unsafe fn autoContentAccessingProxy(&self) -> Retained<AnyObject>;
    }

    unsafe impl NSObjectNSDiscardableContentProxy for NSObject {}
);

#[cfg(feature = "NSZone")]
#[inline]
pub unsafe extern "C-unwind" fn NSAllocateObject(
    a_class: &AnyClass,
    extra_bytes: NSUInteger,
    zone: *mut NSZone,
) -> Retained<AnyObject> {
    extern "C-unwind" {
        fn NSAllocateObject(
            a_class: &AnyClass,
            extra_bytes: NSUInteger,
            zone: *mut NSZone,
        ) -> NonNull<AnyObject>;
    }
    let ret = unsafe { NSAllocateObject(a_class, extra_bytes, zone) };
    unsafe { Retained::retain_autoreleased(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

extern "C-unwind" {
    pub fn NSDeallocateObject(object: &AnyObject);
}

#[cfg(feature = "NSZone")]
#[deprecated = "Not supported"]
#[inline]
pub unsafe extern "C-unwind" fn NSCopyObject(
    object: &AnyObject,
    extra_bytes: NSUInteger,
    zone: *mut NSZone,
) -> Retained<AnyObject> {
    extern "C-unwind" {
        fn NSCopyObject(
            object: &AnyObject,
            extra_bytes: NSUInteger,
            zone: *mut NSZone,
        ) -> NonNull<AnyObject>;
    }
    let ret = unsafe { NSCopyObject(object, extra_bytes, zone) };
    unsafe { Retained::from_raw(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

#[cfg(feature = "NSZone")]
#[inline]
pub unsafe extern "C-unwind" fn NSShouldRetainWithZone(
    an_object: &AnyObject,
    requested_zone: *mut NSZone,
) -> bool {
    extern "C-unwind" {
        fn NSShouldRetainWithZone(an_object: &AnyObject, requested_zone: *mut NSZone) -> Bool;
    }
    unsafe { NSShouldRetainWithZone(an_object, requested_zone) }.as_bool()
}

extern "C-unwind" {
    pub fn NSIncrementExtraRefCount(object: &AnyObject);
}

#[inline]
pub unsafe extern "C-unwind" fn NSDecrementExtraRefCountWasZero(object: &AnyObject) -> bool {
    extern "C-unwind" {
        fn NSDecrementExtraRefCountWasZero(object: &AnyObject) -> Bool;
    }
    unsafe { NSDecrementExtraRefCountWasZero(object) }.as_bool()
}

extern "C-unwind" {
    pub fn NSExtraRefCount(object: &AnyObject) -> NSUInteger;
}
