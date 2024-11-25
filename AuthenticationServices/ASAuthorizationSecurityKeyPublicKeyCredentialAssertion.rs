//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationsecuritykeypublickeycredentialassertion?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialAssertion;
);

#[cfg(feature = "ASAuthorizationCredential")]
unsafe impl ASAuthorizationCredential for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

#[cfg(all(
    feature = "ASAuthorizationCredential",
    feature = "ASAuthorizationPublicKeyCredentialAssertion",
    feature = "ASPublicKeyCredential"
))]
unsafe impl ASAuthorizationPublicKeyCredentialAssertion
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion
{
}

#[cfg(all(
    feature = "ASAuthorizationCredential",
    feature = "ASPublicKeyCredential"
))]
unsafe impl ASPublicKeyCredential for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

unsafe impl NSCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

unsafe impl NSCopying for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

unsafe impl CopyingHelper for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

unsafe impl NSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

extern_methods!(
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {
        #[method(appID)]
        pub unsafe fn appID(&self) -> bool;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
