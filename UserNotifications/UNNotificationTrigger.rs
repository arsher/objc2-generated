//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotificationtrigger?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationTrigger;
);

unsafe impl NSCoding for UNNotificationTrigger {}

unsafe impl NSCopying for UNNotificationTrigger {}

unsafe impl CopyingHelper for UNNotificationTrigger {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UNNotificationTrigger {}

unsafe impl NSSecureCoding for UNNotificationTrigger {}

extern_methods!(
    unsafe impl UNNotificationTrigger {
        #[method(repeats)]
        pub unsafe fn repeats(&self) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unpushnotificationtrigger?language=objc)
    #[unsafe(super(UNNotificationTrigger, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNPushNotificationTrigger;
);

unsafe impl NSCoding for UNPushNotificationTrigger {}

unsafe impl NSCopying for UNPushNotificationTrigger {}

unsafe impl CopyingHelper for UNPushNotificationTrigger {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UNPushNotificationTrigger {}

unsafe impl NSSecureCoding for UNPushNotificationTrigger {}

extern_methods!(
    unsafe impl UNPushNotificationTrigger {}
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    unsafe impl UNPushNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNPushNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/untimeintervalnotificationtrigger?language=objc)
    #[unsafe(super(UNNotificationTrigger, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNTimeIntervalNotificationTrigger;
);

unsafe impl NSCoding for UNTimeIntervalNotificationTrigger {}

unsafe impl NSCopying for UNTimeIntervalNotificationTrigger {}

unsafe impl CopyingHelper for UNTimeIntervalNotificationTrigger {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UNTimeIntervalNotificationTrigger {}

unsafe impl NSSecureCoding for UNTimeIntervalNotificationTrigger {}

extern_methods!(
    unsafe impl UNTimeIntervalNotificationTrigger {
        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        #[method_id(@__retain_semantics Other triggerWithTimeInterval:repeats:)]
        pub unsafe fn triggerWithTimeInterval_repeats(
            time_interval: NSTimeInterval,
            repeats: bool,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other nextTriggerDate)]
        pub unsafe fn nextTriggerDate(&self) -> Option<Retained<NSDate>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    unsafe impl UNTimeIntervalNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNTimeIntervalNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/uncalendarnotificationtrigger?language=objc)
    #[unsafe(super(UNNotificationTrigger, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNCalendarNotificationTrigger;
);

unsafe impl NSCoding for UNCalendarNotificationTrigger {}

unsafe impl NSCopying for UNCalendarNotificationTrigger {}

unsafe impl CopyingHelper for UNCalendarNotificationTrigger {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UNCalendarNotificationTrigger {}

unsafe impl NSSecureCoding for UNCalendarNotificationTrigger {}

extern_methods!(
    unsafe impl UNCalendarNotificationTrigger {
        #[method_id(@__retain_semantics Other dateComponents)]
        pub unsafe fn dateComponents(&self) -> Retained<NSDateComponents>;

        #[method_id(@__retain_semantics Other triggerWithDateMatchingComponents:repeats:)]
        pub unsafe fn triggerWithDateMatchingComponents_repeats(
            date_components: &NSDateComponents,
            repeats: bool,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other nextTriggerDate)]
        pub unsafe fn nextTriggerDate(&self) -> Option<Retained<NSDate>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    unsafe impl UNCalendarNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNCalendarNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unlocationnotificationtrigger?language=objc)
    #[unsafe(super(UNNotificationTrigger, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNLocationNotificationTrigger;
);

unsafe impl NSCoding for UNLocationNotificationTrigger {}

unsafe impl NSCopying for UNLocationNotificationTrigger {}

unsafe impl CopyingHelper for UNLocationNotificationTrigger {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UNLocationNotificationTrigger {}

unsafe impl NSSecureCoding for UNLocationNotificationTrigger {}

extern_methods!(
    unsafe impl UNLocationNotificationTrigger {
        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other region)]
        pub unsafe fn region(&self) -> Retained<CLRegion>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other triggerWithRegion:repeats:)]
        pub unsafe fn triggerWithRegion_repeats(region: &CLRegion, repeats: bool)
            -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    unsafe impl UNLocationNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNLocationNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
