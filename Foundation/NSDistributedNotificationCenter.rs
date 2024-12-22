//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdistributednotificationcentertype?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSDistributedNotificationCenterType = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalnotificationcentertype?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocalNotificationCenterType: &'static NSDistributedNotificationCenterType;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnotificationsuspensionbehavior?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSNotificationSuspensionBehavior(pub NSUInteger);
impl NSNotificationSuspensionBehavior {
    #[doc(alias = "NSNotificationSuspensionBehaviorDrop")]
    pub const Drop: Self = Self(1);
    #[doc(alias = "NSNotificationSuspensionBehaviorCoalesce")]
    pub const Coalesce: Self = Self(2);
    #[doc(alias = "NSNotificationSuspensionBehaviorHold")]
    pub const Hold: Self = Self(3);
    #[doc(alias = "NSNotificationSuspensionBehaviorDeliverImmediately")]
    pub const DeliverImmediately: Self = Self(4);
}

unsafe impl Encode for NSNotificationSuspensionBehavior {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSNotificationSuspensionBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdistributednotificationoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDistributedNotificationOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSDistributedNotificationOptions: NSUInteger {
        const NSDistributedNotificationDeliverImmediately = 1<<0;
        const NSDistributedNotificationPostToAllSessions = 1<<1;
    }
}

unsafe impl Encode for NSDistributedNotificationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDistributedNotificationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnotificationdeliverimmediately?language=objc)
pub static NSNotificationDeliverImmediately: NSDistributedNotificationOptions =
    NSDistributedNotificationOptions(
        NSDistributedNotificationOptions::NSDistributedNotificationDeliverImmediately.0,
    );

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnotificationposttoallsessions?language=objc)
pub static NSNotificationPostToAllSessions: NSDistributedNotificationOptions =
    NSDistributedNotificationOptions(
        NSDistributedNotificationOptions::NSDistributedNotificationPostToAllSessions.0,
    );

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdistributednotificationcenter?language=objc)
    #[unsafe(super(NSNotificationCenter, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSNotification")]
    pub struct NSDistributedNotificationCenter;
);

#[cfg(feature = "NSNotification")]
unsafe impl NSObjectProtocol for NSDistributedNotificationCenter {}

extern_methods!(
    #[cfg(feature = "NSNotification")]
    unsafe impl NSDistributedNotificationCenter {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other notificationCenterForType:)]
        pub unsafe fn notificationCenterForType(
            notification_center_type: &NSDistributedNotificationCenterType,
        ) -> Retained<NSDistributedNotificationCenter>;

        #[method_id(@__retain_semantics Other defaultCenter)]
        pub unsafe fn defaultCenter() -> Retained<NSDistributedNotificationCenter>;

        #[cfg(feature = "NSString")]
        #[method(addObserver:selector:name:object:suspensionBehavior:)]
        pub unsafe fn addObserver_selector_name_object_suspensionBehavior(
            &self,
            observer: &AnyObject,
            selector: Sel,
            name: Option<&NSNotificationName>,
            object: Option<&NSString>,
            suspension_behavior: NSNotificationSuspensionBehavior,
        );

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(postNotificationName:object:userInfo:deliverImmediately:)]
        pub unsafe fn postNotificationName_object_userInfo_deliverImmediately(
            &self,
            name: &NSNotificationName,
            object: Option<&NSString>,
            user_info: Option<&NSDictionary>,
            deliver_immediately: bool,
        );

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(postNotificationName:object:userInfo:options:)]
        pub unsafe fn postNotificationName_object_userInfo_options(
            &self,
            name: &NSNotificationName,
            object: Option<&NSString>,
            user_info: Option<&NSDictionary>,
            options: NSDistributedNotificationOptions,
        );

        #[method(suspended)]
        pub unsafe fn suspended(&self) -> bool;

        /// Setter for [`suspended`][Self::suspended].
        #[method(setSuspended:)]
        pub unsafe fn setSuspended(&self, suspended: bool);

        #[cfg(feature = "NSString")]
        #[method(addObserver:selector:name:object:)]
        pub unsafe fn addObserver_selector_name_object(
            &self,
            observer: &AnyObject,
            a_selector: Sel,
            a_name: Option<&NSNotificationName>,
            an_object: Option<&NSString>,
        );

        #[cfg(feature = "NSString")]
        #[method(postNotificationName:object:)]
        pub unsafe fn postNotificationName_object(
            &self,
            a_name: &NSNotificationName,
            an_object: Option<&NSString>,
        );

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(postNotificationName:object:userInfo:)]
        pub unsafe fn postNotificationName_object_userInfo(
            &self,
            a_name: &NSNotificationName,
            an_object: Option<&NSString>,
            a_user_info: Option<&NSDictionary>,
        );

        #[cfg(feature = "NSString")]
        #[method(removeObserver:name:object:)]
        pub unsafe fn removeObserver_name_object(
            &self,
            observer: &AnyObject,
            a_name: Option<&NSNotificationName>,
            an_object: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSNotification")]
    unsafe impl NSDistributedNotificationCenter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
