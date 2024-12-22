//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aspasskeyregistrationcredential?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyRegistrationCredential;
);

#[cfg(feature = "ASAuthorizationCredential")]
unsafe impl ASAuthorizationCredential for ASPasskeyRegistrationCredential {}

unsafe impl NSCoding for ASPasskeyRegistrationCredential {}

unsafe impl NSCopying for ASPasskeyRegistrationCredential {}

unsafe impl CopyingHelper for ASPasskeyRegistrationCredential {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASPasskeyRegistrationCredential {}

unsafe impl NSSecureCoding for ASPasskeyRegistrationCredential {}

extern_methods!(
    unsafe impl ASPasskeyRegistrationCredential {
        /// Initializes an ASPasskeyRegistrationCredential object.
        ///
        /// Parameter `relyingParty`: The relying party identifier associated with this passkey.
        ///
        /// Parameter `clientDataHash`: The JSON encoded client data for this registration result.
        ///
        /// Parameter `credentialID`: The unique identifier for this passkey.
        ///
        /// Parameter `attestationObject`: The attestation object for this passkey registration result.
        #[method_id(@__retain_semantics Init initWithRelyingParty:clientDataHash:credentialID:attestationObject:)]
        pub unsafe fn initWithRelyingParty_clientDataHash_credentialID_attestationObject(
            this: Allocated<Self>,
            relying_party: &NSString,
            client_data_hash: &NSData,
            credential_id: &NSData,
            attestation_object: &NSData,
        ) -> Retained<Self>;

        #[cfg(feature = "ASPasskeyRegistrationCredentialExtensionOutput")]
        /// Initializes an ASPasskeyRegistrationCredential object.
        ///
        /// Parameter `relyingParty`: The relying party identifier associated with this passkey.
        ///
        /// Parameter `clientDataHash`: The JSON encoded client data for this registration result.
        ///
        /// Parameter `credentialID`: The unique identifier for this passkey.
        ///
        /// Parameter `attestationObject`: The attestation object for this passkey registration result.
        ///
        /// Parameter `extensionOutput`: The output of WebAuthn extensions processed by the credential provider.
        #[method_id(@__retain_semantics Init initWithRelyingParty:clientDataHash:credentialID:attestationObject:extensionOutput:)]
        pub unsafe fn initWithRelyingParty_clientDataHash_credentialID_attestationObject_extensionOutput(
            this: Allocated<Self>,
            relying_party: &NSString,
            client_data_hash: &NSData,
            credential_id: &NSData,
            attestation_object: &NSData,
            extension_output: Option<&ASPasskeyRegistrationCredentialExtensionOutput>,
        ) -> Retained<Self>;

        /// Creates and initializes an ASPasskeyRegistrationCredential object.
        ///
        /// Parameter `relyingParty`: The relying party identifier associated with this passkey.
        ///
        /// Parameter `clientDataHash`: The JSON encoded client data for this registration result.
        ///
        /// Parameter `credentialID`: The unique identifier for this passkey.
        ///
        /// Parameter `attestationObject`: The attestation object for this passkey registration result.
        #[method_id(@__retain_semantics Other credentialWithRelyingParty:clientDataHash:credentialID:attestationObject:)]
        pub unsafe fn credentialWithRelyingParty_clientDataHash_credentialID_attestationObject(
            relying_party: &NSString,
            client_data_hash: &NSData,
            credential_id: &NSData,
            attestation_object: &NSData,
        ) -> Retained<Self>;

        /// The relying party identifier associated with this passkey.
        #[method_id(@__retain_semantics Other relyingParty)]
        pub unsafe fn relyingParty(&self) -> Retained<NSString>;

        /// The hash of the client data for this registration result.
        #[method_id(@__retain_semantics Other clientDataHash)]
        pub unsafe fn clientDataHash(&self) -> Retained<NSData>;

        /// The raw credential identifier of this passkey.
        #[method_id(@__retain_semantics Other credentialID)]
        pub unsafe fn credentialID(&self) -> Retained<NSData>;

        /// The attestation object for this passkey registration result.
        #[method_id(@__retain_semantics Other attestationObject)]
        pub unsafe fn attestationObject(&self) -> Retained<NSData>;

        #[cfg(feature = "ASPasskeyRegistrationCredentialExtensionOutput")]
        /// The outputs for WebAuthn extensions processed by the credential provider.
        #[method_id(@__retain_semantics Other extensionOutput)]
        pub unsafe fn extensionOutput(
            &self,
        ) -> Option<Retained<ASPasskeyRegistrationCredentialExtensionOutput>>;

        #[cfg(feature = "ASPasskeyRegistrationCredentialExtensionOutput")]
        /// Setter for [`extensionOutput`][Self::extensionOutput].
        #[method(setExtensionOutput:)]
        pub unsafe fn setExtensionOutput(
            &self,
            extension_output: Option<&ASPasskeyRegistrationCredentialExtensionOutput>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASPasskeyRegistrationCredential {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
