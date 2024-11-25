//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aswebauthenticationsessionwebbrowsersessionmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASWebAuthenticationSessionWebBrowserSessionManager;
);

unsafe impl NSObjectProtocol for ASWebAuthenticationSessionWebBrowserSessionManager {}

extern_methods!(
    unsafe impl ASWebAuthenticationSessionWebBrowserSessionManager {
        #[method_id(@__retain_semantics Other sharedManager)]
        pub unsafe fn sharedManager() -> Retained<ASWebAuthenticationSessionWebBrowserSessionManager>;

        #[cfg(feature = "ASWebAuthenticationSessionWebBrowserSessionHandling")]
        #[method_id(@__retain_semantics Other sessionHandler)]
        pub unsafe fn sessionHandler(
            &self,
        ) -> Retained<ProtocolObject<dyn ASWebAuthenticationSessionWebBrowserSessionHandling>>;

        #[cfg(feature = "ASWebAuthenticationSessionWebBrowserSessionHandling")]
        #[method(setSessionHandler:)]
        pub unsafe fn setSessionHandler(
            &self,
            session_handler: &ProtocolObject<
                dyn ASWebAuthenticationSessionWebBrowserSessionHandling,
            >,
        );

        #[method(wasLaunchedByAuthenticationServices)]
        pub unsafe fn wasLaunchedByAuthenticationServices(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASWebAuthenticationSessionWebBrowserSessionManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
