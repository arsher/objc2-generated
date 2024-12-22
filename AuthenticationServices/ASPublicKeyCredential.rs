//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// The base protocol for all PublicKeyCredential credential types.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aspublickeycredential?language=objc)
    #[cfg(feature = "ASAuthorizationCredential")]
    pub unsafe trait ASPublicKeyCredential: ASAuthorizationCredential {
        /// A byte sequence containing the serialized clientDataJSON blob returned by the authenticator.
        #[method_id(@__retain_semantics Other rawClientDataJSON)]
        unsafe fn rawClientDataJSON(&self) -> Retained<NSData>;

        /// An identifier that uniquely identifies this credential.
        #[method_id(@__retain_semantics Other credentialID)]
        unsafe fn credentialID(&self) -> Retained<NSData>;
    }

    #[cfg(feature = "ASAuthorizationCredential")]
    unsafe impl ProtocolType for dyn ASPublicKeyCredential {}
);
