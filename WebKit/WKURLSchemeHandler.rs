//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;

use crate::*;

extern_protocol!(
    /// A class conforming to the WKURLSchemeHandler protocol provides methods for
    /// loading resources with URL schemes that WebKit doesn't know how to handle itself.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wkurlschemehandler?language=objc)
    pub unsafe trait WKURLSchemeHandler: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "WKURLSchemeTask",
            feature = "WKWebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        /// Notifies your app to start loading the data for a particular resource
        /// represented by the URL scheme handler task.
        ///
        /// Parameter `webView`: The web view invoking the method.
        ///
        /// Parameter `urlSchemeTask`: The task that your app should start loading data for.
        #[method(webView:startURLSchemeTask:)]
        unsafe fn webView_startURLSchemeTask(
            &self,
            web_view: &WKWebView,
            url_scheme_task: &ProtocolObject<dyn WKURLSchemeTask>,
        );

        #[cfg(all(
            feature = "WKURLSchemeTask",
            feature = "WKWebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        /// Notifies your app to stop handling a URL scheme handler task.
        ///
        /// Parameter `webView`: The web view invoking the method.
        ///
        /// Parameter `urlSchemeTask`: The task that your app should stop handling.
        ///
        /// After your app is told to stop loading data for a URL scheme handler task
        /// it must not perform any callbacks for that task.
        /// An exception will be thrown if any callbacks are made on the URL scheme handler task
        /// after your app has been told to stop loading for it.
        #[method(webView:stopURLSchemeTask:)]
        unsafe fn webView_stopURLSchemeTask(
            &self,
            web_view: &WKWebView,
            url_scheme_task: &ProtocolObject<dyn WKURLSchemeTask>,
        );
    }

    unsafe impl ProtocolType for dyn WKURLSchemeHandler {}
);
