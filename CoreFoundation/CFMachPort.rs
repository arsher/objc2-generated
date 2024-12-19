//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmachportref?language=objc)
pub type CFMachPortRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmachportcontext?language=objc)
#[cfg(feature = "CFBase")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFMachPortContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<unsafe extern "C-unwind" fn(*mut c_void) -> *mut c_void>,
    pub release: Option<unsafe extern "C-unwind" fn(*mut c_void)>,
    pub copyDescription: Option<unsafe extern "C-unwind" fn(*mut c_void) -> CFStringRef>,
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFMachPortContext {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <CFIndex>::ENCODING,
            <*mut c_void>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(*mut c_void) -> *mut c_void>>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(*mut c_void)>>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(*mut c_void) -> CFStringRef>>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFMachPortContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmachportcallback?language=objc)
#[cfg(feature = "CFBase")]
pub type CFMachPortCallBack =
    Option<unsafe extern "C-unwind" fn(CFMachPortRef, *mut c_void, CFIndex, *mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmachportinvalidationcallback?language=objc)
pub type CFMachPortInvalidationCallBack =
    Option<unsafe extern "C-unwind" fn(CFMachPortRef, *mut c_void)>;

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFMachPortGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFMachPortCreate(
        allocator: CFAllocatorRef,
        callout: CFMachPortCallBack,
        context: *mut CFMachPortContext,
        should_free_info: *mut Boolean,
    ) -> CFMachPortRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "libc"))]
    pub fn CFMachPortCreateWithPort(
        allocator: CFAllocatorRef,
        port_num: libc::mach_port_t,
        callout: CFMachPortCallBack,
        context: *mut CFMachPortContext,
        should_free_info: *mut Boolean,
    ) -> CFMachPortRef;
}

extern "C-unwind" {
    #[cfg(feature = "libc")]
    pub fn CFMachPortGetPort(port: CFMachPortRef) -> libc::mach_port_t;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFMachPortGetContext(port: CFMachPortRef, context: *mut CFMachPortContext);
}

extern "C-unwind" {
    pub fn CFMachPortInvalidate(port: CFMachPortRef);
}

extern "C-unwind" {
    pub fn CFMachPortIsValid(port: CFMachPortRef) -> Boolean;
}

extern "C-unwind" {
    pub fn CFMachPortGetInvalidationCallBack(port: CFMachPortRef)
        -> CFMachPortInvalidationCallBack;
}

extern "C-unwind" {
    pub fn CFMachPortSetInvalidationCallBack(
        port: CFMachPortRef,
        callout: CFMachPortInvalidationCallBack,
    );
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFRunLoop"))]
    pub fn CFMachPortCreateRunLoopSource(
        allocator: CFAllocatorRef,
        port: CFMachPortRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
}
