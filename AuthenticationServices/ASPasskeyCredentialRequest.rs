//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyCredentialRequest;

    unsafe impl ClassType for ASPasskeyCredentialRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "ASCredentialRequest")]
unsafe impl ASCredentialRequest for ASPasskeyCredentialRequest {}

unsafe impl NSCoding for ASPasskeyCredentialRequest {}

unsafe impl NSCopying for ASPasskeyCredentialRequest {}

unsafe impl NSObjectProtocol for ASPasskeyCredentialRequest {}

unsafe impl NSSecureCoding for ASPasskeyCredentialRequest {}

extern_methods!(
    unsafe impl ASPasskeyCredentialRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "ASAuthorizationPublicKeyCredentialConstants",
            feature = "ASPasskeyCredentialIdentity"
        ))]
        #[method_id(@__retain_semantics Init initWithCredentialIdentity:clientDataHash:userVerificationPreference:supportedAlgorithms:)]
        pub unsafe fn initWithCredentialIdentity_clientDataHash_userVerificationPreference_supportedAlgorithms(
            this: Allocated<Self>,
            credential_identity: &ASPasskeyCredentialIdentity,
            client_data_hash: &NSData,
            user_verification_preference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
            supported_algorithms: &NSArray<NSNumber>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "ASAuthorizationPublicKeyCredentialConstants",
            feature = "ASPasskeyCredentialIdentity"
        ))]
        #[method_id(@__retain_semantics Other requestWithCredentialIdentity:clientDataHash:userVerificationPreference:supportedAlgorithms:)]
        pub unsafe fn requestWithCredentialIdentity_clientDataHash_userVerificationPreference_supportedAlgorithms(
            credential_identity: &ASPasskeyCredentialIdentity,
            client_data_hash: &NSData,
            user_verification_preference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
            supported_algorithms: &NSArray<NSNumber>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other clientDataHash)]
        pub unsafe fn clientDataHash(&self) -> Retained<NSData>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialConstants")]
        #[method_id(@__retain_semantics Other userVerificationPreference)]
        pub unsafe fn userVerificationPreference(
            &self,
        ) -> Retained<ASAuthorizationPublicKeyCredentialUserVerificationPreference>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialConstants")]
        #[method(setUserVerificationPreference:)]
        pub unsafe fn setUserVerificationPreference(
            &self,
            user_verification_preference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
        );

        #[method_id(@__retain_semantics Other supportedAlgorithms)]
        pub unsafe fn supportedAlgorithms(&self) -> Retained<NSArray<NSNumber>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASPasskeyCredentialRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
