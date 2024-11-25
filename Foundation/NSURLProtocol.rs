//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlprotocolclient?language=objc)
    pub unsafe trait NSURLProtocolClient: NSObjectProtocol {
        #[cfg(all(feature = "NSURLRequest", feature = "NSURLResponse"))]
        #[method(URLProtocol:wasRedirectedToRequest:redirectResponse:)]
        unsafe fn URLProtocol_wasRedirectedToRequest_redirectResponse(
            &self,
            protocol: &NSURLProtocol,
            request: &NSURLRequest,
            redirect_response: &NSURLResponse,
        );

        #[cfg(feature = "NSURLCache")]
        #[method(URLProtocol:cachedResponseIsValid:)]
        unsafe fn URLProtocol_cachedResponseIsValid(
            &self,
            protocol: &NSURLProtocol,
            cached_response: &NSCachedURLResponse,
        );

        #[cfg(all(feature = "NSURLCache", feature = "NSURLResponse"))]
        #[method(URLProtocol:didReceiveResponse:cacheStoragePolicy:)]
        unsafe fn URLProtocol_didReceiveResponse_cacheStoragePolicy(
            &self,
            protocol: &NSURLProtocol,
            response: &NSURLResponse,
            policy: NSURLCacheStoragePolicy,
        );

        #[cfg(feature = "NSData")]
        #[method(URLProtocol:didLoadData:)]
        unsafe fn URLProtocol_didLoadData(&self, protocol: &NSURLProtocol, data: &NSData);

        #[method(URLProtocolDidFinishLoading:)]
        unsafe fn URLProtocolDidFinishLoading(&self, protocol: &NSURLProtocol);

        #[cfg(feature = "NSError")]
        #[method(URLProtocol:didFailWithError:)]
        unsafe fn URLProtocol_didFailWithError(&self, protocol: &NSURLProtocol, error: &NSError);

        #[cfg(feature = "NSURLAuthenticationChallenge")]
        #[method(URLProtocol:didReceiveAuthenticationChallenge:)]
        unsafe fn URLProtocol_didReceiveAuthenticationChallenge(
            &self,
            protocol: &NSURLProtocol,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(feature = "NSURLAuthenticationChallenge")]
        #[method(URLProtocol:didCancelAuthenticationChallenge:)]
        unsafe fn URLProtocol_didCancelAuthenticationChallenge(
            &self,
            protocol: &NSURLProtocol,
            challenge: &NSURLAuthenticationChallenge,
        );
    }

    unsafe impl ProtocolType for dyn NSURLProtocolClient {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlprotocol?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLProtocol;
);

unsafe impl NSObjectProtocol for NSURLProtocol {}

extern_methods!(
    unsafe impl NSURLProtocol {
        #[cfg(all(feature = "NSURLCache", feature = "NSURLRequest"))]
        #[method_id(@__retain_semantics Init initWithRequest:cachedResponse:client:)]
        pub unsafe fn initWithRequest_cachedResponse_client(
            this: Allocated<Self>,
            request: &NSURLRequest,
            cached_response: Option<&NSCachedURLResponse>,
            client: Option<&ProtocolObject<dyn NSURLProtocolClient>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Retained<ProtocolObject<dyn NSURLProtocolClient>>>;

        #[cfg(feature = "NSURLRequest")]
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Retained<NSURLRequest>;

        #[cfg(feature = "NSURLCache")]
        #[method_id(@__retain_semantics Other cachedResponse)]
        pub unsafe fn cachedResponse(&self) -> Option<Retained<NSCachedURLResponse>>;

        #[cfg(feature = "NSURLRequest")]
        #[method(canInitWithRequest:)]
        pub unsafe fn canInitWithRequest(request: &NSURLRequest) -> bool;

        #[cfg(feature = "NSURLRequest")]
        #[method_id(@__retain_semantics Other canonicalRequestForRequest:)]
        pub unsafe fn canonicalRequestForRequest(request: &NSURLRequest) -> Retained<NSURLRequest>;

        #[cfg(feature = "NSURLRequest")]
        #[method(requestIsCacheEquivalent:toRequest:)]
        pub unsafe fn requestIsCacheEquivalent_toRequest(
            a: &NSURLRequest,
            b: &NSURLRequest,
        ) -> bool;

        #[method(startLoading)]
        pub unsafe fn startLoading(&self);

        #[method(stopLoading)]
        pub unsafe fn stopLoading(&self);

        #[cfg(all(feature = "NSString", feature = "NSURLRequest"))]
        #[method_id(@__retain_semantics Other propertyForKey:inRequest:)]
        pub unsafe fn propertyForKey_inRequest(
            key: &NSString,
            request: &NSURLRequest,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSString", feature = "NSURLRequest"))]
        #[method(setProperty:forKey:inRequest:)]
        pub unsafe fn setProperty_forKey_inRequest(
            value: &AnyObject,
            key: &NSString,
            request: &NSMutableURLRequest,
        );

        #[cfg(all(feature = "NSString", feature = "NSURLRequest"))]
        #[method(removePropertyForKey:inRequest:)]
        pub unsafe fn removePropertyForKey_inRequest(key: &NSString, request: &NSMutableURLRequest);

        #[method(registerClass:)]
        pub unsafe fn registerClass(protocol_class: &AnyClass) -> bool;

        #[method(unregisterClass:)]
        pub unsafe fn unregisterClass(protocol_class: &AnyClass);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSURLProtocol {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSURLSessionTaskAdditions
    unsafe impl NSURLProtocol {
        #[cfg(feature = "NSURLSession")]
        #[method(canInitWithTask:)]
        pub unsafe fn canInitWithTask(task: &NSURLSessionTask) -> bool;

        #[cfg(all(feature = "NSURLCache", feature = "NSURLSession"))]
        #[method_id(@__retain_semantics Init initWithTask:cachedResponse:client:)]
        pub unsafe fn initWithTask_cachedResponse_client(
            this: Allocated<Self>,
            task: &NSURLSessionTask,
            cached_response: Option<&NSCachedURLResponse>,
            client: Option<&ProtocolObject<dyn NSURLProtocolClient>>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSURLSession")]
        #[method_id(@__retain_semantics Other task)]
        pub unsafe fn task(&self) -> Option<Retained<NSURLSessionTask>>;
    }
);
