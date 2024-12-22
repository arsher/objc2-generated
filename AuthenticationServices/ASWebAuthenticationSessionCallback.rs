//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An object used to evaluate navigation events in an authentication session. When the session navigates
    /// to a matching URL, it will pass the URL to the session completion handler.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aswebauthenticationsessioncallback?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASWebAuthenticationSessionCallback;
);

unsafe impl Send for ASWebAuthenticationSessionCallback {}

unsafe impl Sync for ASWebAuthenticationSessionCallback {}

unsafe impl NSObjectProtocol for ASWebAuthenticationSessionCallback {}

extern_methods!(
    unsafe impl ASWebAuthenticationSessionCallback {
        /// Creates a callback object that matches against URLs with the given custom scheme.
        ///
        /// Parameter `customScheme`: The custom scheme that the app expects in the callback URL.
        #[method_id(@__retain_semantics Other callbackWithCustomScheme:)]
        pub unsafe fn callbackWithCustomScheme(custom_scheme: &NSString) -> Retained<Self>;

        /// Creates a callback object that matches against HTTPS URLs with the given host and path.
        ///
        /// Parameter `host`: The host that the app expects in the callback URL. The host must be associated with the
        /// app using associated web credentials domains.
        ///
        /// Parameter `path`: The path that the app expects in the callback URL.
        #[method_id(@__retain_semantics Other callbackWithHTTPSHost:path:)]
        pub unsafe fn callbackWithHTTPSHost_path(
            host: &NSString,
            path: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Check whether a given main-frame navigation URL matches the callback expected by the client app. Handles all URL-based callback strategies, including custom schemes and HTTPS navigations.
        /// This is mainly meant for web browsers adopting the ASWebAuthenticationWebBrowser API, but may also be useful for other apps for debugging purposes.
        ///
        /// Parameter `url`: The URL to check.
        #[method(matchesURL:)]
        pub unsafe fn matchesURL(&self, url: &NSURL) -> bool;
    }
);
