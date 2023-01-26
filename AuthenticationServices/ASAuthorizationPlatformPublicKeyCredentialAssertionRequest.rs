//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertionRequest"
    )]
    pub struct ASAuthorizationPlatformPublicKeyCredentialAssertionRequest;

    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertionRequest"
    )]
    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
    }
);

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertionRequest"
)]
unsafe impl ASAuthorizationPublicKeyCredentialAssertionRequest
    for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest
{
}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertionRequest"
)]
unsafe impl NSCoding for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertionRequest"
)]
unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertionRequest"
)]
unsafe impl NSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertionRequest"
    )]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other allowedCredentials)]
        pub unsafe fn allowedCredentials(
            &self,
        ) -> Id<NSArray<ASAuthorizationPlatformPublicKeyCredentialDescriptor>, Shared>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method(setAllowedCredentials:)]
        pub unsafe fn setAllowedCredentials(
            &self,
            allowed_credentials: &NSArray<ASAuthorizationPlatformPublicKeyCredentialDescriptor>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
