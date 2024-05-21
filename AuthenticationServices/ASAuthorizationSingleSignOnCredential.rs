//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationSingleSignOnCredential;

    unsafe impl ClassType for ASAuthorizationSingleSignOnCredential {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "ASAuthorizationCredential")]
unsafe impl ASAuthorizationCredential for ASAuthorizationSingleSignOnCredential {}

unsafe impl NSCoding for ASAuthorizationSingleSignOnCredential {}

unsafe impl NSCopying for ASAuthorizationSingleSignOnCredential {}

unsafe impl NSObjectProtocol for ASAuthorizationSingleSignOnCredential {}

unsafe impl NSSecureCoding for ASAuthorizationSingleSignOnCredential {}

extern_methods!(
    unsafe impl ASAuthorizationSingleSignOnCredential {
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other accessToken)]
        pub unsafe fn accessToken(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other identityToken)]
        pub unsafe fn identityToken(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "ASAuthorization")]
        #[method_id(@__retain_semantics Other authorizedScopes)]
        pub unsafe fn authorizedScopes(&self) -> Retained<NSArray<ASAuthorizationScope>>;

        #[method_id(@__retain_semantics Other authenticatedResponse)]
        pub unsafe fn authenticatedResponse(&self) -> Option<Retained<NSHTTPURLResponse>>;

        #[method_id(@__retain_semantics Other privateKeys)]
        pub unsafe fn privateKeys(&self) -> Retained<NSArray>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
