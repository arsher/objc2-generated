//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_protocol!(
    #[deprecated]
    pub struct WebResourceLoadDelegate;

    unsafe impl ProtocolType for WebResourceLoadDelegate {
        #[cfg(all(
            feature = "Foundation_NSURLRequest",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other webView:identifierForInitialRequest:fromDataSource:)]
        pub unsafe fn webView_identifierForInitialRequest_fromDataSource(
            &self,
            sender: Option<&WebView>,
            request: Option<&NSURLRequest>,
            data_source: Option<&WebDataSource>,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSURLRequest",
            feature = "Foundation_NSURLResponse",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other webView:resource:willSendRequest:redirectResponse:fromDataSource:)]
        pub unsafe fn webView_resource_willSendRequest_redirectResponse_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&Object>,
            request: Option<&NSURLRequest>,
            redirect_response: Option<&NSURLResponse>,
            data_source: Option<&WebDataSource>,
        ) -> Option<Id<NSURLRequest, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:resource:didReceiveAuthenticationChallenge:fromDataSource:)]
        pub unsafe fn webView_resource_didReceiveAuthenticationChallenge_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&Object>,
            challenge: Option<&NSURLAuthenticationChallenge>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:resource:didCancelAuthenticationChallenge:fromDataSource:)]
        pub unsafe fn webView_resource_didCancelAuthenticationChallenge_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&Object>,
            challenge: Option<&NSURLAuthenticationChallenge>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "Foundation_NSURLResponse",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:resource:didReceiveResponse:fromDataSource:)]
        pub unsafe fn webView_resource_didReceiveResponse_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&Object>,
            response: Option<&NSURLResponse>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(feature = "WebKit_WebDataSource", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:resource:didReceiveContentLength:fromDataSource:)]
        pub unsafe fn webView_resource_didReceiveContentLength_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&Object>,
            length: NSInteger,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(feature = "WebKit_WebDataSource", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:resource:didFinishLoadingFromDataSource:)]
        pub unsafe fn webView_resource_didFinishLoadingFromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&Object>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:resource:didFailLoadingWithError:fromDataSource:)]
        pub unsafe fn webView_resource_didFailLoadingWithError_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&Object>,
            error: Option<&NSError>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:plugInFailedWithError:dataSource:)]
        pub unsafe fn webView_plugInFailedWithError_dataSource(
            &self,
            sender: Option<&WebView>,
            error: Option<&NSError>,
            data_source: Option<&WebDataSource>,
        );
    }
);