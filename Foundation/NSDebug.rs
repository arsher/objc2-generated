//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdebugenabled?language=objc)
    pub static NSDebugEnabled: Bool;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nszombieenabled?language=objc)
    pub static NSZombieEnabled: Bool;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdeallocatezombies?language=objc)
    pub static NSDeallocateZombies: Bool;
}

#[inline]
pub unsafe extern "C-unwind" fn NSIsFreedObject(an_object: Option<&AnyObject>) -> bool {
    extern "C-unwind" {
        fn NSIsFreedObject(an_object: Option<&AnyObject>) -> Bool;
    }
    unsafe { NSIsFreedObject(an_object) }.as_bool()
}

extern "C-unwind" {
    pub fn NSFrameAddress(frame: NSUInteger) -> *mut c_void;
}

extern "C-unwind" {
    pub fn NSReturnAddress(frame: NSUInteger) -> *mut c_void;
}

extern "C-unwind" {
    pub fn NSCountFrames() -> NSUInteger;
}

extern_methods!(
    /// NSAutoreleasePoolDebugging
    #[cfg(feature = "NSAutoreleasePool")]
    unsafe impl NSAutoreleasePool {
        #[method(showPools)]
        pub unsafe fn showPools();
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nskeepallocationstatistics?language=objc)
    pub static NSKeepAllocationStatistics: Bool;
}

extern "C-unwind" {
    pub fn NSRecordAllocationEvent(event_type: c_int, object: Option<&AnyObject>);
}
