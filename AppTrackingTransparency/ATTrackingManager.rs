//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/apptrackingtransparency/attrackingmanagerauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ATTrackingManagerAuthorizationStatus(pub NSUInteger);
impl ATTrackingManagerAuthorizationStatus {
    #[doc(alias = "ATTrackingManagerAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "ATTrackingManagerAuthorizationStatusRestricted")]
    pub const Restricted: Self = Self(1);
    #[doc(alias = "ATTrackingManagerAuthorizationStatusDenied")]
    pub const Denied: Self = Self(2);
    #[doc(alias = "ATTrackingManagerAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
}

unsafe impl Encode for ATTrackingManagerAuthorizationStatus {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for ATTrackingManagerAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/apptrackingtransparency/attrackingmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ATTrackingManager;
);

unsafe impl NSObjectProtocol for ATTrackingManager {}

extern_methods!(
    unsafe impl ATTrackingManager {
        #[method(trackingAuthorizationStatus)]
        pub unsafe fn trackingAuthorizationStatus() -> ATTrackingManagerAuthorizationStatus;

        #[cfg(feature = "block2")]
        #[method(requestTrackingAuthorizationWithCompletionHandler:)]
        pub unsafe fn requestTrackingAuthorizationWithCompletionHandler(
            completion: &block2::Block<dyn Fn(ATTrackingManagerAuthorizationStatus)>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
