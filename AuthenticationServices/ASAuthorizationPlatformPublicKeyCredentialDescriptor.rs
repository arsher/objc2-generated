//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationplatformpublickeycredentialdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPlatformPublicKeyCredentialDescriptor;
);

#[cfg(feature = "ASAuthorizationPublicKeyCredentialDescriptor")]
unsafe impl ASAuthorizationPublicKeyCredentialDescriptor
    for ASAuthorizationPlatformPublicKeyCredentialDescriptor
{
}

unsafe impl NSCoding for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}

unsafe impl NSCopying for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}

unsafe impl CopyingHelper for ASAuthorizationPlatformPublicKeyCredentialDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}

unsafe impl NSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}

extern_methods!(
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialDescriptor {
        #[method_id(@__retain_semantics Init initWithCredentialID:)]
        pub unsafe fn initWithCredentialID(
            this: Allocated<Self>,
            credential_id: &NSData,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
