//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotificationattributedmessagecontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationAttributedMessageContext;
);

unsafe impl NSObjectProtocol for UNNotificationAttributedMessageContext {}

#[cfg(feature = "UNNotificationContent")]
unsafe impl UNNotificationContentProviding for UNNotificationAttributedMessageContext {}

extern_methods!(
    unsafe impl UNNotificationAttributedMessageContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationAttributedMessageContext {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
