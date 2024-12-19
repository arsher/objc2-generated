//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aspasskeycredentialrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyCredentialRequest;
);

#[cfg(feature = "ASCredentialRequest")]
unsafe impl ASCredentialRequest for ASPasskeyCredentialRequest {}

unsafe impl NSCoding for ASPasskeyCredentialRequest {}

unsafe impl NSCopying for ASPasskeyCredentialRequest {}

unsafe impl CopyingHelper for ASPasskeyCredentialRequest {
    type Result = Self;
}

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
            feature = "ASPasskeyAssertionCredentialExtensionInput",
            feature = "ASPasskeyCredentialIdentity"
        ))]
        #[method_id(@__retain_semantics Init initWithCredentialIdentity:clientDataHash:userVerificationPreference:supportedAlgorithms:assertionExtensionInput:)]
        pub unsafe fn initWithCredentialIdentity_clientDataHash_userVerificationPreference_supportedAlgorithms_assertionExtensionInput(
            this: Allocated<Self>,
            credential_identity: &ASPasskeyCredentialIdentity,
            client_data_hash: &NSData,
            user_verification_preference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
            supported_algorithms: &NSArray<NSNumber>,
            assertion_extension_input: Option<&ASPasskeyAssertionCredentialExtensionInput>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "ASAuthorizationPublicKeyCredentialConstants",
            feature = "ASPasskeyCredentialIdentity",
            feature = "ASPasskeyRegistrationCredentialExtensionInput"
        ))]
        #[method_id(@__retain_semantics Init initWithCredentialIdentity:clientDataHash:userVerificationPreference:supportedAlgorithms:registrationExtensionInput:)]
        pub unsafe fn initWithCredentialIdentity_clientDataHash_userVerificationPreference_supportedAlgorithms_registrationExtensionInput(
            this: Allocated<Self>,
            credential_identity: &ASPasskeyCredentialIdentity,
            client_data_hash: &NSData,
            user_verification_preference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
            supported_algorithms: &NSArray<NSNumber>,
            registration_extension_input: Option<&ASPasskeyRegistrationCredentialExtensionInput>,
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

        #[cfg(feature = "ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
        #[method_id(@__retain_semantics Other excludedCredentials)]
        pub unsafe fn excludedCredentials(
            &self,
        ) -> Option<Retained<NSArray<ASAuthorizationPlatformPublicKeyCredentialDescriptor>>>;

        #[cfg(feature = "ASPasskeyAssertionCredentialExtensionInput")]
        #[method_id(@__retain_semantics Other assertionExtensionInput)]
        pub unsafe fn assertionExtensionInput(
            &self,
        ) -> Option<Retained<ASPasskeyAssertionCredentialExtensionInput>>;

        #[cfg(feature = "ASPasskeyRegistrationCredentialExtensionInput")]
        #[method_id(@__retain_semantics Other registrationExtensionInput)]
        pub unsafe fn registrationExtensionInput(
            &self,
        ) -> Option<Retained<ASPasskeyRegistrationCredentialExtensionInput>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASPasskeyCredentialRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
