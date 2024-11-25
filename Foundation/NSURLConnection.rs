//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlconnection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLConnection;
);

unsafe impl NSObjectProtocol for NSURLConnection {}

extern_methods!(
    unsafe impl NSURLConnection {
        #[cfg(feature = "NSURLRequest")]
        #[deprecated = "Use NSURLSession (see NSURLSession.h)"]
        #[method_id(@__retain_semantics Init initWithRequest:delegate:startImmediately:)]
        pub unsafe fn initWithRequest_delegate_startImmediately(
            this: Allocated<Self>,
            request: &NSURLRequest,
            delegate: Option<&AnyObject>,
            start_immediately: bool,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSURLRequest")]
        #[deprecated = "Use NSURLSession (see NSURLSession.h)"]
        #[method_id(@__retain_semantics Init initWithRequest:delegate:)]
        pub unsafe fn initWithRequest_delegate(
            this: Allocated<Self>,
            request: &NSURLRequest,
            delegate: Option<&AnyObject>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSURLRequest")]
        #[deprecated = "Use NSURLSession (see NSURLSession.h)"]
        #[method_id(@__retain_semantics Other connectionWithRequest:delegate:)]
        pub unsafe fn connectionWithRequest_delegate(
            request: &NSURLRequest,
            delegate: Option<&AnyObject>,
        ) -> Option<Retained<NSURLConnection>>;

        #[cfg(feature = "NSURLRequest")]
        #[method_id(@__retain_semantics Other originalRequest)]
        pub unsafe fn originalRequest(&self) -> Retained<NSURLRequest>;

        #[cfg(feature = "NSURLRequest")]
        #[method_id(@__retain_semantics Other currentRequest)]
        pub unsafe fn currentRequest(&self) -> Retained<NSURLRequest>;

        #[method(start)]
        pub unsafe fn start(&self);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRunLoop", feature = "NSString"))]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRunLoop", feature = "NSString"))]
        #[method(unscheduleFromRunLoop:forMode:)]
        pub unsafe fn unscheduleFromRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[cfg(feature = "NSOperation")]
        #[method(setDelegateQueue:)]
        pub unsafe fn setDelegateQueue(&self, queue: Option<&NSOperationQueue>);

        #[cfg(feature = "NSURLRequest")]
        #[method(canHandleRequest:)]
        pub unsafe fn canHandleRequest(request: &NSURLRequest) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSURLConnection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlconnectiondelegate?language=objc)
    pub unsafe trait NSURLConnectionDelegate: NSObjectProtocol {
        #[cfg(feature = "NSError")]
        #[optional]
        #[method(connection:didFailWithError:)]
        unsafe fn connection_didFailWithError(&self, connection: &NSURLConnection, error: &NSError);

        #[optional]
        #[method(connectionShouldUseCredentialStorage:)]
        unsafe fn connectionShouldUseCredentialStorage(&self, connection: &NSURLConnection)
            -> bool;

        #[cfg(feature = "NSURLAuthenticationChallenge")]
        #[optional]
        #[method(connection:willSendRequestForAuthenticationChallenge:)]
        unsafe fn connection_willSendRequestForAuthenticationChallenge(
            &self,
            connection: &NSURLConnection,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(feature = "NSURLProtectionSpace")]
        #[deprecated = "Use -connection:willSendRequestForAuthenticationChallenge: instead."]
        #[optional]
        #[method(connection:canAuthenticateAgainstProtectionSpace:)]
        unsafe fn connection_canAuthenticateAgainstProtectionSpace(
            &self,
            connection: &NSURLConnection,
            protection_space: &NSURLProtectionSpace,
        ) -> bool;

        #[cfg(feature = "NSURLAuthenticationChallenge")]
        #[deprecated = "Use -connection:willSendRequestForAuthenticationChallenge: instead."]
        #[optional]
        #[method(connection:didReceiveAuthenticationChallenge:)]
        unsafe fn connection_didReceiveAuthenticationChallenge(
            &self,
            connection: &NSURLConnection,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(feature = "NSURLAuthenticationChallenge")]
        #[deprecated = "Use -connection:willSendRequestForAuthenticationChallenge: instead."]
        #[optional]
        #[method(connection:didCancelAuthenticationChallenge:)]
        unsafe fn connection_didCancelAuthenticationChallenge(
            &self,
            connection: &NSURLConnection,
            challenge: &NSURLAuthenticationChallenge,
        );
    }

    unsafe impl ProtocolType for dyn NSURLConnectionDelegate {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlconnectiondatadelegate?language=objc)
    pub unsafe trait NSURLConnectionDataDelegate: NSURLConnectionDelegate {
        #[cfg(all(feature = "NSURLRequest", feature = "NSURLResponse"))]
        #[optional]
        #[method_id(@__retain_semantics Other connection:willSendRequest:redirectResponse:)]
        unsafe fn connection_willSendRequest_redirectResponse(
            &self,
            connection: &NSURLConnection,
            request: &NSURLRequest,
            response: Option<&NSURLResponse>,
        ) -> Option<Retained<NSURLRequest>>;

        #[cfg(feature = "NSURLResponse")]
        #[optional]
        #[method(connection:didReceiveResponse:)]
        unsafe fn connection_didReceiveResponse(
            &self,
            connection: &NSURLConnection,
            response: &NSURLResponse,
        );

        #[cfg(feature = "NSData")]
        #[optional]
        #[method(connection:didReceiveData:)]
        unsafe fn connection_didReceiveData(&self, connection: &NSURLConnection, data: &NSData);

        #[cfg(all(feature = "NSStream", feature = "NSURLRequest"))]
        #[optional]
        #[method_id(@__retain_semantics Other connection:needNewBodyStream:)]
        unsafe fn connection_needNewBodyStream(
            &self,
            connection: &NSURLConnection,
            request: &NSURLRequest,
        ) -> Option<Retained<NSInputStream>>;

        #[optional]
        #[method(connection:didSendBodyData:totalBytesWritten:totalBytesExpectedToWrite:)]
        unsafe fn connection_didSendBodyData_totalBytesWritten_totalBytesExpectedToWrite(
            &self,
            connection: &NSURLConnection,
            bytes_written: NSInteger,
            total_bytes_written: NSInteger,
            total_bytes_expected_to_write: NSInteger,
        );

        #[cfg(feature = "NSURLCache")]
        #[optional]
        #[method_id(@__retain_semantics Other connection:willCacheResponse:)]
        unsafe fn connection_willCacheResponse(
            &self,
            connection: &NSURLConnection,
            cached_response: &NSCachedURLResponse,
        ) -> Option<Retained<NSCachedURLResponse>>;

        #[optional]
        #[method(connectionDidFinishLoading:)]
        unsafe fn connectionDidFinishLoading(&self, connection: &NSURLConnection);
    }

    unsafe impl ProtocolType for dyn NSURLConnectionDataDelegate {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlconnectiondownloaddelegate?language=objc)
    pub unsafe trait NSURLConnectionDownloadDelegate: NSURLConnectionDelegate {
        #[optional]
        #[method(connection:didWriteData:totalBytesWritten:expectedTotalBytes:)]
        unsafe fn connection_didWriteData_totalBytesWritten_expectedTotalBytes(
            &self,
            connection: &NSURLConnection,
            bytes_written: c_longlong,
            total_bytes_written: c_longlong,
            expected_total_bytes: c_longlong,
        );

        #[optional]
        #[method(connectionDidResumeDownloading:totalBytesWritten:expectedTotalBytes:)]
        unsafe fn connectionDidResumeDownloading_totalBytesWritten_expectedTotalBytes(
            &self,
            connection: &NSURLConnection,
            total_bytes_written: c_longlong,
            expected_total_bytes: c_longlong,
        );

        #[cfg(feature = "NSURL")]
        #[method(connectionDidFinishDownloading:destinationURL:)]
        unsafe fn connectionDidFinishDownloading_destinationURL(
            &self,
            connection: &NSURLConnection,
            destination_url: &NSURL,
        );
    }

    unsafe impl ProtocolType for dyn NSURLConnectionDownloadDelegate {}
);

extern_methods!(
    /// NSURLConnectionSynchronousLoading
    unsafe impl NSURLConnection {
        #[cfg(all(
            feature = "NSData",
            feature = "NSError",
            feature = "NSURLRequest",
            feature = "NSURLResponse"
        ))]
        #[deprecated = "Use [NSURLSession dataTaskWithRequest:completionHandler:] (see NSURLSession.h"]
        #[method_id(@__retain_semantics Other sendSynchronousRequest:returningResponse:error:_)]
        pub unsafe fn sendSynchronousRequest_returningResponse_error(
            request: &NSURLRequest,
            response: Option<&mut Option<Retained<NSURLResponse>>>,
        ) -> Result<Retained<NSData>, Retained<NSError>>;
    }
);

extern_methods!(
    /// NSURLConnectionQueuedLoading
    unsafe impl NSURLConnection {
        #[cfg(all(
            feature = "NSData",
            feature = "NSError",
            feature = "NSOperation",
            feature = "NSURLRequest",
            feature = "NSURLResponse",
            feature = "block2"
        ))]
        #[deprecated = "Use [NSURLSession dataTaskWithRequest:completionHandler:] (see NSURLSession.h"]
        #[method(sendAsynchronousRequest:queue:completionHandler:)]
        pub unsafe fn sendAsynchronousRequest_queue_completionHandler(
            request: &NSURLRequest,
            queue: &NSOperationQueue,
            handler: &block2::Block<dyn Fn(*mut NSURLResponse, *mut NSData, *mut NSError)>,
        );
    }
);
