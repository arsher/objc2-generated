//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyAssertionCredentialExtensionInput;

    unsafe impl ClassType for ASPasskeyAssertionCredentialExtensionInput {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for ASPasskeyAssertionCredentialExtensionInput {}

unsafe impl NSCopying for ASPasskeyAssertionCredentialExtensionInput {}

unsafe impl CopyingHelper for ASPasskeyAssertionCredentialExtensionInput {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASPasskeyAssertionCredentialExtensionInput {}

unsafe impl NSSecureCoding for ASPasskeyAssertionCredentialExtensionInput {}

extern_methods!(
    unsafe impl ASPasskeyAssertionCredentialExtensionInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput")]
        #[method_id(@__retain_semantics Other largeBlob)]
        pub unsafe fn largeBlob(
            &self,
        ) -> Option<Retained<ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput>>;
    }
);