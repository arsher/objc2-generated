//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern "C-unwind" {
    pub fn NSDefaultMallocZone() -> NonNull<NSZone>;
}

#[inline]
pub unsafe extern "C-unwind" fn NSCreateZone(
    start_size: NSUInteger,
    granularity: NSUInteger,
    can_free: bool,
) -> NonNull<NSZone> {
    extern "C-unwind" {
        fn NSCreateZone(
            start_size: NSUInteger,
            granularity: NSUInteger,
            can_free: Bool,
        ) -> NonNull<NSZone>;
    }
    unsafe { NSCreateZone(start_size, granularity, Bool::new(can_free)) }
}

extern "C-unwind" {
    pub fn NSRecycleZone(zone: NonNull<NSZone>);
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSSetZoneName(zone: *mut NSZone, name: &NSString);
}

#[cfg(feature = "NSString")]
#[inline]
pub unsafe extern "C-unwind" fn NSZoneName(zone: *mut NSZone) -> Retained<NSString> {
    extern "C-unwind" {
        fn NSZoneName(zone: *mut NSZone) -> NonNull<NSString>;
    }
    let ret = unsafe { NSZoneName(zone) };
    unsafe { Retained::retain_autoreleased(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

extern "C-unwind" {
    pub fn NSZoneFromPointer(ptr: NonNull<c_void>) -> *mut NSZone;
}

extern "C-unwind" {
    pub fn NSZoneMalloc(zone: *mut NSZone, size: NSUInteger) -> NonNull<c_void>;
}

extern "C-unwind" {
    pub fn NSZoneCalloc(
        zone: *mut NSZone,
        num_elems: NSUInteger,
        byte_size: NSUInteger,
    ) -> NonNull<c_void>;
}

extern "C-unwind" {
    pub fn NSZoneRealloc(zone: *mut NSZone, ptr: *mut c_void, size: NSUInteger) -> NonNull<c_void>;
}

extern "C-unwind" {
    pub fn NSZoneFree(zone: *mut NSZone, ptr: NonNull<c_void>);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsscannedoption?language=objc)
pub const NSScannedOption: NSUInteger = 1 << 0;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscollectordisabledoption?language=objc)
pub const NSCollectorDisabledOption: NSUInteger = 1 << 1;

extern "C-unwind" {
    pub fn NSAllocateCollectable(size: NSUInteger, options: NSUInteger) -> NonNull<c_void>;
}

extern "C-unwind" {
    pub fn NSReallocateCollectable(
        ptr: *mut c_void,
        size: NSUInteger,
        options: NSUInteger,
    ) -> NonNull<c_void>;
}

extern "C-unwind" {
    pub fn NSPageSize() -> NSUInteger;
}

extern "C-unwind" {
    pub fn NSLogPageSize() -> NSUInteger;
}

extern "C-unwind" {
    pub fn NSRoundUpToMultipleOfPageSize(bytes: NSUInteger) -> NSUInteger;
}

extern "C-unwind" {
    pub fn NSRoundDownToMultipleOfPageSize(bytes: NSUInteger) -> NSUInteger;
}

extern "C-unwind" {
    pub fn NSAllocateMemoryPages(bytes: NSUInteger) -> NonNull<c_void>;
}

extern "C-unwind" {
    pub fn NSDeallocateMemoryPages(ptr: NonNull<c_void>, bytes: NSUInteger);
}

extern "C-unwind" {
    pub fn NSCopyMemoryPages(source: NonNull<c_void>, dest: NonNull<c_void>, bytes: NSUInteger);
}

extern "C-unwind" {
    #[deprecated = "Use NSProcessInfo instead"]
    pub fn NSRealMemoryAvailable() -> NSUInteger;
}
