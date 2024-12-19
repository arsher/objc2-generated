//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketref?language=objc)
pub type CFSocketRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketerror?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFSocketError(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFSocketError {
    pub const kCFSocketSuccess: Self = Self(0);
    pub const kCFSocketError: Self = Self(-1);
    pub const kCFSocketTimeout: Self = Self(-2);
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFSocketError {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFSocketError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketsignature?language=objc)
#[cfg(feature = "CFData")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFSocketSignature {
    pub protocolFamily: i32,
    pub socketType: i32,
    pub protocol: i32,
    pub address: CFDataRef,
}

#[cfg(all(feature = "CFData", feature = "objc2"))]
unsafe impl Encode for CFSocketSignature {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <i32>::ENCODING,
            <i32>::ENCODING,
            <i32>::ENCODING,
            <CFDataRef>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "CFData", feature = "objc2"))]
unsafe impl RefEncode for CFSocketSignature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketcallbacktype?language=objc)
// NS_OPTIONS
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFSocketCallBackType(pub CFOptionFlags);
#[cfg(feature = "CFBase")]
bitflags::bitflags! {
    impl CFSocketCallBackType: CFOptionFlags {
        const kCFSocketNoCallBack = 0;
        const kCFSocketReadCallBack = 1;
        const kCFSocketAcceptCallBack = 2;
        const kCFSocketDataCallBack = 3;
        const kCFSocketConnectCallBack = 4;
        const kCFSocketWriteCallBack = 8;
    }
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFSocketCallBackType {
    const ENCODING: Encoding = CFOptionFlags::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFSocketCallBackType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketautomaticallyreenablereadcallback?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketAutomaticallyReenableReadCallBack: CFOptionFlags = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketautomaticallyreenableacceptcallback?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketAutomaticallyReenableAcceptCallBack: CFOptionFlags = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketautomaticallyreenabledatacallback?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketAutomaticallyReenableDataCallBack: CFOptionFlags = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketautomaticallyreenablewritecallback?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketAutomaticallyReenableWriteCallBack: CFOptionFlags = 8;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketleaveerrors?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketLeaveErrors: CFOptionFlags = 64;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketcloseoninvalidate?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFSocketCloseOnInvalidate: CFOptionFlags = 128;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketcallback?language=objc)
#[cfg(all(feature = "CFBase", feature = "CFData"))]
pub type CFSocketCallBack = Option<
    unsafe extern "C-unwind" fn(
        CFSocketRef,
        CFSocketCallBackType,
        CFDataRef,
        *mut c_void,
        *mut c_void,
    ),
>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketcontext?language=objc)
#[cfg(feature = "CFBase")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFSocketContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<unsafe extern "C-unwind" fn(*mut c_void) -> *mut c_void>,
    pub release: Option<unsafe extern "C-unwind" fn(*mut c_void)>,
    pub copyDescription: Option<unsafe extern "C-unwind" fn(*mut c_void) -> CFStringRef>,
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFSocketContext {
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
unsafe impl RefEncode for CFSocketContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketnativehandle?language=objc)
pub type CFSocketNativeHandle = c_int;

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFSocketGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData"))]
    pub fn CFSocketCreate(
        allocator: CFAllocatorRef,
        protocol_family: i32,
        socket_type: i32,
        protocol: i32,
        call_back_types: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *mut CFSocketContext,
    ) -> CFSocketRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData"))]
    pub fn CFSocketCreateWithNative(
        allocator: CFAllocatorRef,
        sock: CFSocketNativeHandle,
        call_back_types: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *mut CFSocketContext,
    ) -> CFSocketRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData"))]
    pub fn CFSocketCreateWithSocketSignature(
        allocator: CFAllocatorRef,
        signature: *mut CFSocketSignature,
        call_back_types: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *mut CFSocketContext,
    ) -> CFSocketRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketCreateConnectedToSocketSignature(
        allocator: CFAllocatorRef,
        signature: *mut CFSocketSignature,
        call_back_types: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *mut CFSocketContext,
        timeout: CFTimeInterval,
    ) -> CFSocketRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData"))]
    pub fn CFSocketSetAddress(s: CFSocketRef, address: CFDataRef) -> CFSocketError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketConnectToAddress(
        s: CFSocketRef,
        address: CFDataRef,
        timeout: CFTimeInterval,
    ) -> CFSocketError;
}

extern "C-unwind" {
    pub fn CFSocketInvalidate(s: CFSocketRef);
}

extern "C-unwind" {
    pub fn CFSocketIsValid(s: CFSocketRef) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFData")]
    pub fn CFSocketCopyAddress(s: CFSocketRef) -> CFDataRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFData")]
    pub fn CFSocketCopyPeerAddress(s: CFSocketRef) -> CFDataRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFSocketGetContext(s: CFSocketRef, context: *mut CFSocketContext);
}

extern "C-unwind" {
    pub fn CFSocketGetNative(s: CFSocketRef) -> CFSocketNativeHandle;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFRunLoop"))]
    pub fn CFSocketCreateRunLoopSource(
        allocator: CFAllocatorRef,
        s: CFSocketRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFSocketGetSocketFlags(s: CFSocketRef) -> CFOptionFlags;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFSocketSetSocketFlags(s: CFSocketRef, flags: CFOptionFlags);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFSocketDisableCallBacks(s: CFSocketRef, call_back_types: CFOptionFlags);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFSocketEnableCallBacks(s: CFSocketRef, call_back_types: CFOptionFlags);
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketSendData(
        s: CFSocketRef,
        address: CFDataRef,
        data: CFDataRef,
        timeout: CFTimeInterval,
    ) -> CFSocketError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketRegisterValue(
        name_server_signature: *mut CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        value: CFPropertyListRef,
    ) -> CFSocketError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketCopyRegisteredValue(
        name_server_signature: *mut CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        value: *mut CFPropertyListRef,
        name_server_address: *mut CFDataRef,
    ) -> CFSocketError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketRegisterSocketSignature(
        name_server_signature: *mut CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        signature: *mut CFSocketSignature,
    ) -> CFSocketError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketCopyRegisteredSocketSignature(
        name_server_signature: *mut CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        signature: *mut CFSocketSignature,
        name_server_address: *mut CFDataRef,
    ) -> CFSocketError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData", feature = "CFDate"))]
    pub fn CFSocketUnregister(
        name_server_signature: *mut CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
    ) -> CFSocketError;
}

extern "C-unwind" {
    pub fn CFSocketSetDefaultNameRegistryPortNumber(port: u16);
}

extern "C-unwind" {
    pub fn CFSocketGetDefaultNameRegistryPortNumber() -> u16;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketcommandkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketCommandKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketnamekey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketNameKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketvaluekey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketValueKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketresultkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketResultKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketerrorkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketErrorKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketregistercommand?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketRegisterCommand: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfsocketretrievecommand?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFSocketRetrieveCommand: CFStringRef;
}
