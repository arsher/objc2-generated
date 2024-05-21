//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation(pub NSInteger);
impl ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation {
    #[doc(alias = "ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperationRead")]
    pub const Read: Self = Self(0);
    #[doc(alias = "ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperationWrite")]
    pub const Write: Self = Self(1);
}

unsafe impl Encode for ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput;

    unsafe impl ClassType for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {}

extern_methods!(
    unsafe impl ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithOperation:)]
        pub unsafe fn initWithOperation(
            this: Allocated<Self>,
            operation: ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation,
        ) -> Retained<Self>;

        #[method(operation)]
        pub unsafe fn operation(
            &self,
        ) -> ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation;

        #[method_id(@__retain_semantics Other dataToWrite)]
        pub unsafe fn dataToWrite(&self) -> Option<Retained<NSData>>;

        #[method(setDataToWrite:)]
        pub unsafe fn setDataToWrite(&self, data_to_write: Option<&NSData>);
    }
);
