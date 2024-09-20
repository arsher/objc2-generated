//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPublicKeyCredentialPRFRegistrationOutput;

    unsafe impl ClassType for ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {}

extern_methods!(
    unsafe impl ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {
        #[method(isSupported)]
        pub unsafe fn isSupported(&self) -> bool;

        #[method_id(@__retain_semantics Other first)]
        pub unsafe fn first(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other second)]
        pub unsafe fn second(&self) -> Option<Retained<NSData>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);