//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    pub unsafe trait ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialRegistrationRequest {
        #[cfg(feature = "ASPublicKeyCredentialClientData")]
        #[method_id(@__retain_semantics Other clientData)]
        unsafe fn clientData(&self) -> Option<Retained<ASPublicKeyCredentialClientData>>;
    }

    unsafe impl ProtocolType
        for dyn ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialRegistrationRequest
    {
    }
);
