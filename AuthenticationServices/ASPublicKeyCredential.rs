//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aspublickeycredential?language=objc)
    #[cfg(feature = "ASAuthorizationCredential")]
    pub unsafe trait ASPublicKeyCredential: ASAuthorizationCredential {
        #[method_id(@__retain_semantics Other rawClientDataJSON)]
        unsafe fn rawClientDataJSON(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other credentialID)]
        unsafe fn credentialID(&self) -> Retained<NSData>;
    }

    #[cfg(feature = "ASAuthorizationCredential")]
    unsafe impl ProtocolType for dyn ASPublicKeyCredential {}
);
