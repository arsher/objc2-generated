//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation {
        ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperationRead = 0,
        ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperationWrite = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput"
    )]
    pub struct ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput;

    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput"
    )]
    unsafe impl ClassType for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput")]
unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {}

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput"
    )]
    unsafe impl ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOperation:)]
        pub unsafe fn initWithOperation(
            this: Allocated<Self>,
            operation: ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation,
        ) -> Id<Self>;

        #[method(operation)]
        pub unsafe fn operation(
            &self,
        ) -> ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other dataToWrite)]
        pub unsafe fn dataToWrite(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setDataToWrite:)]
        pub unsafe fn setDataToWrite(&self, data_to_write: Option<&NSData>);
    }
);