//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationwebbrowserplatformpublickeycredentialregistrationrequest?language=objc)
    pub unsafe trait ASAuthorizationWebBrowserPlatformPublicKeyCredentialRegistrationRequest {
        #[cfg(feature = "ASPublicKeyCredentialClientData")]
        #[method_id(@__retain_semantics Other clientData)]
        unsafe fn clientData(&self) -> Option<Retained<ASPublicKeyCredentialClientData>>;

        #[cfg(feature = "ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
        #[method_id(@__retain_semantics Other excludedCredentials)]
        unsafe fn excludedCredentials(
            &self,
        ) -> Option<Retained<NSArray<ASAuthorizationPlatformPublicKeyCredentialDescriptor>>>;

        #[cfg(feature = "ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
        #[method(setExcludedCredentials:)]
        unsafe fn setExcludedCredentials(
            &self,
            excluded_credentials: Option<
                &NSArray<ASAuthorizationPlatformPublicKeyCredentialDescriptor>,
            >,
        );

        #[method(shouldShowHybridTransport)]
        unsafe fn shouldShowHybridTransport(&self) -> bool;

        #[method(setShouldShowHybridTransport:)]
        unsafe fn setShouldShowHybridTransport(&self, should_show_hybrid_transport: bool);
    }

    unsafe impl ProtocolType
        for dyn ASAuthorizationWebBrowserPlatformPublicKeyCredentialRegistrationRequest
    {
    }
);
