//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationscope?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type ASAuthorizationScope = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationscopefullname?language=objc)
    pub static ASAuthorizationScopeFullName: &'static ASAuthorizationScope;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationscopeemail?language=objc)
    pub static ASAuthorizationScopeEmail: &'static ASAuthorizationScope;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorization?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorization;
);

unsafe impl NSObjectProtocol for ASAuthorization {}

extern_methods!(
    unsafe impl ASAuthorization {
        #[cfg(feature = "ASAuthorizationProvider")]
        #[method_id(@__retain_semantics Other provider)]
        pub unsafe fn provider(&self) -> Retained<ProtocolObject<dyn ASAuthorizationProvider>>;

        #[cfg(feature = "ASAuthorizationCredential")]
        #[method_id(@__retain_semantics Other credential)]
        pub unsafe fn credential(&self) -> Retained<ProtocolObject<dyn ASAuthorizationCredential>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
