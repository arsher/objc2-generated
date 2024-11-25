//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationplatformpublickeycredentialassertion?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPlatformPublicKeyCredentialAssertion;
);

#[cfg(feature = "ASAuthorizationCredential")]
unsafe impl ASAuthorizationCredential for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

#[cfg(all(
    feature = "ASAuthorizationCredential",
    feature = "ASAuthorizationPublicKeyCredentialAssertion",
    feature = "ASPublicKeyCredential"
))]
unsafe impl ASAuthorizationPublicKeyCredentialAssertion
    for ASAuthorizationPlatformPublicKeyCredentialAssertion
{
}

#[cfg(all(
    feature = "ASAuthorizationCredential",
    feature = "ASPublicKeyCredential"
))]
unsafe impl ASPublicKeyCredential for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

unsafe impl NSCoding for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

unsafe impl NSCopying for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

unsafe impl CopyingHelper for ASAuthorizationPlatformPublicKeyCredentialAssertion {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

unsafe impl NSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

extern_methods!(
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialAssertion {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialConstants")]
        #[method(attachment)]
        pub unsafe fn attachment(&self) -> ASAuthorizationPublicKeyCredentialAttachment;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput")]
        #[method_id(@__retain_semantics Other largeBlob)]
        pub unsafe fn largeBlob(
            &self,
        ) -> Option<Retained<ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput>>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialPRFAssertionOutput")]
        #[method_id(@__retain_semantics Other prf)]
        pub unsafe fn prf(
            &self,
        ) -> Option<Retained<ASAuthorizationPublicKeyCredentialPRFAssertionOutput>>;
    }
);
