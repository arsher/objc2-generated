//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct ASAuthorizationPublicKeyCredentialRegistration;

    unsafe impl ProtocolType for ASAuthorizationPublicKeyCredentialRegistration {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other rawAttestationObject)]
        pub unsafe fn rawAttestationObject(&self) -> Option<Id<NSData, Shared>>;
    }
);
