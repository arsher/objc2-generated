//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialassertion?language=objc)
    #[cfg(all(
        feature = "ASAuthorizationCredential",
        feature = "ASPublicKeyCredential"
    ))]
    pub unsafe trait ASAuthorizationPublicKeyCredentialAssertion:
        ASPublicKeyCredential
    {
        /// A byte sequence containing the encoded authenticatorData blob returned by the authenticator.
        #[method_id(@__retain_semantics Other rawAuthenticatorData)]
        unsafe fn rawAuthenticatorData(&self) -> Retained<NSData>;

        /// The userID provided when creating this credential.
        #[method_id(@__retain_semantics Other userID)]
        unsafe fn userID(&self) -> Retained<NSData>;

        /// The signature provided by the authenticator using the credential's private key.
        #[method_id(@__retain_semantics Other signature)]
        unsafe fn signature(&self) -> Retained<NSData>;
    }

    #[cfg(all(
        feature = "ASAuthorizationCredential",
        feature = "ASPublicKeyCredential"
    ))]
    unsafe impl ProtocolType for dyn ASAuthorizationPublicKeyCredentialAssertion {}
);
