//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmsimplequeueerror_allocationfailed?language=objc)
pub const kCMSimpleQueueError_AllocationFailed: OSStatus = -12770;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmsimplequeueerror_requiredparametermissing?language=objc)
pub const kCMSimpleQueueError_RequiredParameterMissing: OSStatus = -12771;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmsimplequeueerror_parameteroutofrange?language=objc)
pub const kCMSimpleQueueError_ParameterOutOfRange: OSStatus = -12772;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmsimplequeueerror_queueisfull?language=objc)
pub const kCMSimpleQueueError_QueueIsFull: OSStatus = -12773;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmsimplequeueref?language=objc)
pub type CMSimpleQueueRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMSimpleQueueGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMSimpleQueueCreate(
        allocator: CFAllocatorRef,
        capacity: i32,
        queue_out: NonNull<CMSimpleQueueRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMSimpleQueueEnqueue(queue: CMSimpleQueueRef, element: NonNull<c_void>) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMSimpleQueueDequeue(queue: CMSimpleQueueRef) -> *mut c_void;
}

extern "C-unwind" {
    pub fn CMSimpleQueueGetHead(queue: CMSimpleQueueRef) -> *mut c_void;
}

extern "C-unwind" {
    pub fn CMSimpleQueueReset(queue: CMSimpleQueueRef) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMSimpleQueueGetCapacity(queue: CMSimpleQueueRef) -> i32;
}

extern "C-unwind" {
    pub fn CMSimpleQueueGetCount(queue: CMSimpleQueueRef) -> i32;
}
