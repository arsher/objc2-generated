//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webresourceloaddelegate?language=objc)
    #[deprecated]
    pub unsafe trait WebResourceLoadDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "WebDataSource",
            feature = "WebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webView:identifierForInitialRequest:fromDataSource:)]
        unsafe fn webView_identifierForInitialRequest_fromDataSource(
            &self,
            sender: Option<&WebView>,
            request: Option<&NSURLRequest>,
            data_source: Option<&WebDataSource>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(
            feature = "WebDataSource",
            feature = "WebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webView:resource:willSendRequest:redirectResponse:fromDataSource:)]
        unsafe fn webView_resource_willSendRequest_redirectResponse_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            request: Option<&NSURLRequest>,
            redirect_response: Option<&NSURLResponse>,
            data_source: Option<&WebDataSource>,
        ) -> Option<Retained<NSURLRequest>>;

        #[cfg(all(
            feature = "WebDataSource",
            feature = "WebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didReceiveAuthenticationChallenge:fromDataSource:)]
        unsafe fn webView_resource_didReceiveAuthenticationChallenge_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            challenge: Option<&NSURLAuthenticationChallenge>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "WebDataSource",
            feature = "WebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didCancelAuthenticationChallenge:fromDataSource:)]
        unsafe fn webView_resource_didCancelAuthenticationChallenge_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            challenge: Option<&NSURLAuthenticationChallenge>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "WebDataSource",
            feature = "WebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didReceiveResponse:fromDataSource:)]
        unsafe fn webView_resource_didReceiveResponse_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            response: Option<&NSURLResponse>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "WebDataSource",
            feature = "WebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didReceiveContentLength:fromDataSource:)]
        unsafe fn webView_resource_didReceiveContentLength_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            length: NSInteger,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "WebDataSource",
            feature = "WebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didFinishLoadingFromDataSource:)]
        unsafe fn webView_resource_didFinishLoadingFromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "WebDataSource",
            feature = "WebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didFailLoadingWithError:fromDataSource:)]
        unsafe fn webView_resource_didFailLoadingWithError_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            error: Option<&NSError>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "WebDataSource",
            feature = "WebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:plugInFailedWithError:dataSource:)]
        unsafe fn webView_plugInFailedWithError_dataSource(
            &self,
            sender: Option<&WebView>,
            error: Option<&NSError>,
            data_source: Option<&WebDataSource>,
        );
    }

    unsafe impl ProtocolType for dyn WebResourceLoadDelegate {}
);
