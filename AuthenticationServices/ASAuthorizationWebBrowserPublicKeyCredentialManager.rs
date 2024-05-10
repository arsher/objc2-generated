//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationState(pub NSInteger);
impl ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationState {
    #[doc(
        alias = "ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationStateAuthorized"
    )]
    pub const Authorized: Self = Self(0);
    #[doc(alias = "ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationStateDenied")]
    pub const Denied: Self = Self(1);
    #[doc(
        alias = "ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationStateNotDetermined"
    )]
    pub const NotDetermined: Self = Self(2);
}

unsafe impl Encode for ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationWebBrowserPublicKeyCredentialManager;

    unsafe impl ClassType for ASAuthorizationWebBrowserPublicKeyCredentialManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for ASAuthorizationWebBrowserPublicKeyCredentialManager {}

unsafe impl Sync for ASAuthorizationWebBrowserPublicKeyCredentialManager {}

unsafe impl NSObjectProtocol for ASAuthorizationWebBrowserPublicKeyCredentialManager {}

extern_methods!(
    unsafe impl ASAuthorizationWebBrowserPublicKeyCredentialManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method(requestAuthorizationForPublicKeyCredentials:)]
        pub unsafe fn requestAuthorizationForPublicKeyCredentials(
            &self,
            completion_handler: &block2::Block<
                dyn Fn(ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationState),
            >,
        );

        #[cfg(all(
            feature = "ASAuthorizationWebBrowserPlatformPublicKeyCredential",
            feature = "block2"
        ))]
        #[method(platformCredentialsForRelyingParty:completionHandler:)]
        pub unsafe fn platformCredentialsForRelyingParty_completionHandler(
            &self,
            relying_party: &NSString,
            completion_handler: &block2::Block<
                dyn Fn(NonNull<NSArray<ASAuthorizationWebBrowserPlatformPublicKeyCredential>>),
            >,
        );

        #[method(authorizationStateForPlatformCredentials)]
        pub unsafe fn authorizationStateForPlatformCredentials(
            &self,
        ) -> ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationState;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationWebBrowserPublicKeyCredentialManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
