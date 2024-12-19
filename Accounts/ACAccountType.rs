//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acaccounttypeidentifiertwitter?language=objc)
    pub static ACAccountTypeIdentifierTwitter: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acaccounttypeidentifierfacebook?language=objc)
    pub static ACAccountTypeIdentifierFacebook: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acaccounttypeidentifiersinaweibo?language=objc)
    pub static ACAccountTypeIdentifierSinaWeibo: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acaccounttypeidentifiertencentweibo?language=objc)
    pub static ACAccountTypeIdentifierTencentWeibo: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acaccounttypeidentifierlinkedin?language=objc)
    pub static ACAccountTypeIdentifierLinkedIn: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acfacebookappidkey?language=objc)
    pub static ACFacebookAppIdKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acfacebookpermissionskey?language=objc)
    pub static ACFacebookPermissionsKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acfacebookaudiencekey?language=objc)
    pub static ACFacebookAudienceKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acfacebookaudienceeveryone?language=objc)
    pub static ACFacebookAudienceEveryone: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acfacebookaudiencefriends?language=objc)
    pub static ACFacebookAudienceFriends: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acfacebookaudienceonlyme?language=objc)
    pub static ACFacebookAudienceOnlyMe: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/aclinkedinappidkey?language=objc)
    pub static ACLinkedInAppIdKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/aclinkedinpermissionskey?language=objc)
    pub static ACLinkedInPermissionsKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/actencentweiboappidkey?language=objc)
    pub static ACTencentWeiboAppIdKey: Option<&'static NSString>;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/accounts/acaccounttype?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
    pub struct ACAccountType;
);

unsafe impl NSObjectProtocol for ACAccountType {}

extern_methods!(
    unsafe impl ACAccountType {
        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method_id(@__retain_semantics Other accountTypeDescription)]
        pub unsafe fn accountTypeDescription(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method(accessGranted)]
        pub unsafe fn accessGranted(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ACAccountType {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
