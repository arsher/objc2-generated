//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationRequest;
);

unsafe impl NSCoding for ASAuthorizationRequest {}

unsafe impl NSCopying for ASAuthorizationRequest {}

unsafe impl CopyingHelper for ASAuthorizationRequest {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASAuthorizationRequest {}

unsafe impl NSSecureCoding for ASAuthorizationRequest {}

extern_methods!(
    unsafe impl ASAuthorizationRequest {
        #[cfg(feature = "ASAuthorizationProvider")]
        #[method_id(@__retain_semantics Other provider)]
        pub unsafe fn provider(&self) -> Retained<ProtocolObject<dyn ASAuthorizationProvider>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
