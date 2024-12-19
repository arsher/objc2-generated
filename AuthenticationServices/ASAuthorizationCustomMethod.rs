//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationcustommethod?language=objc)
// NS_TYPED_ENUM
pub type ASAuthorizationCustomMethod = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationcustommethodvideosubscriberaccount?language=objc)
    pub static ASAuthorizationCustomMethodVideoSubscriberAccount:
        &'static ASAuthorizationCustomMethod;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationcustommethodrestorepurchase?language=objc)
    pub static ASAuthorizationCustomMethodRestorePurchase: &'static ASAuthorizationCustomMethod;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationcustommethodother?language=objc)
    pub static ASAuthorizationCustomMethodOther: &'static ASAuthorizationCustomMethod;
}
