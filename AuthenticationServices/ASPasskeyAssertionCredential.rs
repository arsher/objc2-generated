//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aspasskeyassertioncredential?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyAssertionCredential;
);

#[cfg(feature = "ASAuthorizationCredential")]
unsafe impl ASAuthorizationCredential for ASPasskeyAssertionCredential {}

unsafe impl NSCoding for ASPasskeyAssertionCredential {}

unsafe impl NSCopying for ASPasskeyAssertionCredential {}

unsafe impl CopyingHelper for ASPasskeyAssertionCredential {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASPasskeyAssertionCredential {}

unsafe impl NSSecureCoding for ASPasskeyAssertionCredential {}

extern_methods!(
    unsafe impl ASPasskeyAssertionCredential {
        #[method_id(@__retain_semantics Init initWithUserHandle:relyingParty:signature:clientDataHash:authenticatorData:credentialID:)]
        pub unsafe fn initWithUserHandle_relyingParty_signature_clientDataHash_authenticatorData_credentialID(
            this: Allocated<Self>,
            user_handle: &NSData,
            relying_party: &NSString,
            signature: &NSData,
            client_data_hash: &NSData,
            authenticator_data: &NSData,
            credential_id: &NSData,
        ) -> Retained<Self>;

        #[cfg(feature = "ASPasskeyAssertionCredentialExtensionOutput")]
        #[method_id(@__retain_semantics Init initWithUserHandle:relyingParty:signature:clientDataHash:authenticatorData:credentialID:extensionOutput:)]
        pub unsafe fn initWithUserHandle_relyingParty_signature_clientDataHash_authenticatorData_credentialID_extensionOutput(
            this: Allocated<Self>,
            user_handle: &NSData,
            relying_party: &NSString,
            signature: &NSData,
            client_data_hash: &NSData,
            authenticator_data: &NSData,
            credential_id: &NSData,
            extension_output: Option<&ASPasskeyAssertionCredentialExtensionOutput>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other credentialWithUserHandle:relyingParty:signature:clientDataHash:authenticatorData:credentialID:)]
        pub unsafe fn credentialWithUserHandle_relyingParty_signature_clientDataHash_authenticatorData_credentialID(
            user_handle: &NSData,
            relying_party: &NSString,
            signature: &NSData,
            client_data_hash: &NSData,
            authenticator_data: &NSData,
            credential_id: &NSData,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other userHandle)]
        pub unsafe fn userHandle(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other relyingParty)]
        pub unsafe fn relyingParty(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other signature)]
        pub unsafe fn signature(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other clientDataHash)]
        pub unsafe fn clientDataHash(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other authenticatorData)]
        pub unsafe fn authenticatorData(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other credentialID)]
        pub unsafe fn credentialID(&self) -> Retained<NSData>;

        #[cfg(feature = "ASPasskeyAssertionCredentialExtensionOutput")]
        #[method_id(@__retain_semantics Other extensionOutput)]
        pub unsafe fn extensionOutput(
            &self,
        ) -> Option<Retained<ASPasskeyAssertionCredentialExtensionOutput>>;

        #[cfg(feature = "ASPasskeyAssertionCredentialExtensionOutput")]
        #[method(setExtensionOutput:)]
        pub unsafe fn setExtensionOutput(
            &self,
            extension_output: Option<&ASPasskeyAssertionCredentialExtensionOutput>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASPasskeyAssertionCredential {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
