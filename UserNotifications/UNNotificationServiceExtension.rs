//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotificationserviceextension?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationServiceExtension;
);

unsafe impl NSObjectProtocol for UNNotificationServiceExtension {}

extern_methods!(
    unsafe impl UNNotificationServiceExtension {
        #[cfg(all(
            feature = "UNNotificationContent",
            feature = "UNNotificationRequest",
            feature = "block2"
        ))]
        #[method(didReceiveNotificationRequest:withContentHandler:)]
        pub unsafe fn didReceiveNotificationRequest_withContentHandler(
            &self,
            request: &UNNotificationRequest,
            content_handler: &block2::Block<dyn Fn(NonNull<UNNotificationContent>)>,
        );

        #[method(serviceExtensionTimeWillExpire)]
        pub unsafe fn serviceExtensionTimeWillExpire(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationServiceExtension {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
