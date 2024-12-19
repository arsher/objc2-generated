//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsappleeventmanagersuspensionid?language=objc)
pub type NSAppleEventManagerSuspensionID = *mut c_void;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsappleeventtimeoutdefault?language=objc)
    pub static NSAppleEventTimeOutDefault: c_double;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsappleeventtimeoutnone?language=objc)
    pub static NSAppleEventTimeOutNone: c_double;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsappleeventmanagerwillprocessfirsteventnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSAppleEventManagerWillProcessFirstEventNotification: &'static NSNotificationName;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsappleeventmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAppleEventManager;
);

unsafe impl NSObjectProtocol for NSAppleEventManager {}

extern_methods!(
    unsafe impl NSAppleEventManager {
        #[method_id(@__retain_semantics Other sharedAppleEventManager)]
        pub unsafe fn sharedAppleEventManager() -> Retained<NSAppleEventManager>;

        #[cfg(feature = "NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other currentAppleEvent)]
        pub unsafe fn currentAppleEvent(&self) -> Option<Retained<NSAppleEventDescriptor>>;

        #[cfg(feature = "NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other currentReplyAppleEvent)]
        pub unsafe fn currentReplyAppleEvent(&self) -> Option<Retained<NSAppleEventDescriptor>>;

        #[method(suspendCurrentAppleEvent)]
        pub unsafe fn suspendCurrentAppleEvent(&self) -> NSAppleEventManagerSuspensionID;

        #[cfg(feature = "NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other appleEventForSuspensionID:)]
        pub unsafe fn appleEventForSuspensionID(
            &self,
            suspension_id: NSAppleEventManagerSuspensionID,
        ) -> Retained<NSAppleEventDescriptor>;

        #[cfg(feature = "NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other replyAppleEventForSuspensionID:)]
        pub unsafe fn replyAppleEventForSuspensionID(
            &self,
            suspension_id: NSAppleEventManagerSuspensionID,
        ) -> Retained<NSAppleEventDescriptor>;

        #[method(setCurrentAppleEventAndReplyEventWithSuspensionID:)]
        pub unsafe fn setCurrentAppleEventAndReplyEventWithSuspensionID(
            &self,
            suspension_id: NSAppleEventManagerSuspensionID,
        );

        #[method(resumeWithSuspensionID:)]
        pub unsafe fn resumeWithSuspensionID(&self, suspension_id: NSAppleEventManagerSuspensionID);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAppleEventManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
