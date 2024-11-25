//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asonetimecodecredential?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASOneTimeCodeCredential;
);

#[cfg(feature = "ASAuthorizationCredential")]
unsafe impl ASAuthorizationCredential for ASOneTimeCodeCredential {}

unsafe impl NSCoding for ASOneTimeCodeCredential {}

unsafe impl NSCopying for ASOneTimeCodeCredential {}

unsafe impl CopyingHelper for ASOneTimeCodeCredential {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASOneTimeCodeCredential {}

unsafe impl NSSecureCoding for ASOneTimeCodeCredential {}

extern_methods!(
    unsafe impl ASOneTimeCodeCredential {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other credentialWithCode:)]
        pub unsafe fn credentialWithCode(code: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCode:)]
        pub unsafe fn initWithCode(this: Allocated<Self>, code: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Other code)]
        pub unsafe fn code(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASOneTimeCodeCredential {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
