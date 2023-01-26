//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest"
    )]
    pub struct ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest;

    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest"
    )]
    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
    }
);

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest"
)]
unsafe impl ASAuthorizationPublicKeyCredentialRegistrationRequest
    for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
{
}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest"
)]
unsafe impl NSCoding for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest"
)]
unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest"
)]
unsafe impl NSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest"
    )]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
