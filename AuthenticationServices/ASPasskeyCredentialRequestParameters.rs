//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters")]
    pub struct ASPasskeyCredentialRequestParameters;

    #[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters")]
    unsafe impl ClassType for ASPasskeyCredentialRequestParameters {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters")]
unsafe impl Send for ASPasskeyCredentialRequestParameters {}

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters")]
unsafe impl Sync for ASPasskeyCredentialRequestParameters {}

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters")]
unsafe impl NSCoding for ASPasskeyCredentialRequestParameters {}

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters")]
unsafe impl NSCopying for ASPasskeyCredentialRequestParameters {}

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters")]
unsafe impl NSObjectProtocol for ASPasskeyCredentialRequestParameters {}

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters")]
unsafe impl NSSecureCoding for ASPasskeyCredentialRequestParameters {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters")]
    unsafe impl ASPasskeyCredentialRequestParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other relyingPartyIdentifier)]
        pub unsafe fn relyingPartyIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other clientDataHash)]
        pub unsafe fn clientDataHash(&self) -> Id<NSData>;

        #[method_id(@__retain_semantics Other userVerificationPreference)]
        pub unsafe fn userVerificationPreference(
            &self,
        ) -> Id<ASAuthorizationPublicKeyCredentialUserVerificationPreference>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[method_id(@__retain_semantics Other allowedCredentials)]
        pub unsafe fn allowedCredentials(&self) -> Id<NSArray<NSData>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters")]
    unsafe impl ASPasskeyCredentialRequestParameters {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);