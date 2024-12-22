//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialparameters?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPublicKeyCredentialParameters;
);

unsafe impl NSCoding for ASAuthorizationPublicKeyCredentialParameters {}

unsafe impl NSCopying for ASAuthorizationPublicKeyCredentialParameters {}

unsafe impl CopyingHelper for ASAuthorizationPublicKeyCredentialParameters {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialParameters {}

unsafe impl NSSecureCoding for ASAuthorizationPublicKeyCredentialParameters {}

extern_methods!(
    unsafe impl ASAuthorizationPublicKeyCredentialParameters {
        #[cfg(feature = "ASCOSEConstants")]
        #[method_id(@__retain_semantics Init initWithAlgorithm:)]
        pub unsafe fn initWithAlgorithm(
            this: Allocated<Self>,
            algorithm: ASCOSEAlgorithmIdentifier,
        ) -> Retained<Self>;

        #[cfg(feature = "ASCOSEConstants")]
        /// A COSE algorithm indentifier.
        #[method(algorithm)]
        pub unsafe fn algorithm(&self) -> ASCOSEAlgorithmIdentifier;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
