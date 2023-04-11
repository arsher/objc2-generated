//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSURLProtocolClient: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSURLProtocol",
            feature = "Foundation_NSURLRequest",
            feature = "Foundation_NSURLResponse"
        ))]
        #[method(URLProtocol:wasRedirectedToRequest:redirectResponse:)]
        unsafe fn URLProtocol_wasRedirectedToRequest_redirectResponse(
            &self,
            protocol: &NSURLProtocol,
            request: &NSURLRequest,
            redirect_response: &NSURLResponse,
        );

        #[cfg(all(
            feature = "Foundation_NSCachedURLResponse",
            feature = "Foundation_NSURLProtocol"
        ))]
        #[method(URLProtocol:cachedResponseIsValid:)]
        unsafe fn URLProtocol_cachedResponseIsValid(
            &self,
            protocol: &NSURLProtocol,
            cached_response: &NSCachedURLResponse,
        );

        #[cfg(all(
            feature = "Foundation_NSURLProtocol",
            feature = "Foundation_NSURLResponse"
        ))]
        #[method(URLProtocol:didReceiveResponse:cacheStoragePolicy:)]
        unsafe fn URLProtocol_didReceiveResponse_cacheStoragePolicy(
            &self,
            protocol: &NSURLProtocol,
            response: &NSURLResponse,
            policy: NSURLCacheStoragePolicy,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSURLProtocol"))]
        #[method(URLProtocol:didLoadData:)]
        unsafe fn URLProtocol_didLoadData(&self, protocol: &NSURLProtocol, data: &NSData);

        #[cfg(feature = "Foundation_NSURLProtocol")]
        #[method(URLProtocolDidFinishLoading:)]
        unsafe fn URLProtocolDidFinishLoading(&self, protocol: &NSURLProtocol);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURLProtocol"))]
        #[method(URLProtocol:didFailWithError:)]
        unsafe fn URLProtocol_didFailWithError(&self, protocol: &NSURLProtocol, error: &NSError);

        #[cfg(all(
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "Foundation_NSURLProtocol"
        ))]
        #[method(URLProtocol:didReceiveAuthenticationChallenge:)]
        unsafe fn URLProtocol_didReceiveAuthenticationChallenge(
            &self,
            protocol: &NSURLProtocol,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(all(
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "Foundation_NSURLProtocol"
        ))]
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
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLProtocol")]
    pub struct NSURLProtocol;

    #[cfg(feature = "Foundation_NSURLProtocol")]
    unsafe impl ClassType for NSURLProtocol {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSURLProtocol")]
unsafe impl NSObjectProtocol for NSURLProtocol {}

extern_methods!(
    #[cfg(feature = "Foundation_NSURLProtocol")]
    unsafe impl NSURLProtocol {
        #[cfg(all(
            feature = "Foundation_NSCachedURLResponse",
            feature = "Foundation_NSURLRequest"
        ))]
        #[method_id(@__retain_semantics Init initWithRequest:cachedResponse:client:)]
        pub unsafe fn initWithRequest_cachedResponse_client(
            this: Option<Allocated<Self>>,
            request: &NSURLRequest,
            cached_response: Option<&NSCachedURLResponse>,
            client: Option<&ProtocolObject<dyn NSURLProtocolClient>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Id<ProtocolObject<dyn NSURLProtocolClient>>>;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Id<NSURLRequest>;

        #[cfg(feature = "Foundation_NSCachedURLResponse")]
        #[method_id(@__retain_semantics Other cachedResponse)]
        pub unsafe fn cachedResponse(&self) -> Option<Id<NSCachedURLResponse>>;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method(canInitWithRequest:)]
        pub unsafe fn canInitWithRequest(request: &NSURLRequest) -> bool;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other canonicalRequestForRequest:)]
        pub unsafe fn canonicalRequestForRequest(request: &NSURLRequest) -> Id<NSURLRequest>;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method(requestIsCacheEquivalent:toRequest:)]
        pub unsafe fn requestIsCacheEquivalent_toRequest(
            a: &NSURLRequest,
            b: &NSURLRequest,
        ) -> bool;

        #[method(startLoading)]
        pub unsafe fn startLoading(&self);

        #[method(stopLoading)]
        pub unsafe fn stopLoading(&self);

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURLRequest"))]
        #[method_id(@__retain_semantics Other propertyForKey:inRequest:)]
        pub unsafe fn propertyForKey_inRequest(
            key: &NSString,
            request: &NSURLRequest,
        ) -> Option<Id<Object>>;

        #[cfg(all(
            feature = "Foundation_NSMutableURLRequest",
            feature = "Foundation_NSString"
        ))]
        #[method(setProperty:forKey:inRequest:)]
        pub unsafe fn setProperty_forKey_inRequest(
            value: &Object,
            key: &NSString,
            request: &NSMutableURLRequest,
        );

        #[cfg(all(
            feature = "Foundation_NSMutableURLRequest",
            feature = "Foundation_NSString"
        ))]
        #[method(removePropertyForKey:inRequest:)]
        pub unsafe fn removePropertyForKey_inRequest(key: &NSString, request: &NSMutableURLRequest);

        #[method(registerClass:)]
        pub unsafe fn registerClass(protocol_class: &Class) -> bool;

        #[method(unregisterClass:)]
        pub unsafe fn unregisterClass(protocol_class: &Class);
    }
);

extern_methods!(
    /// NSURLSessionTaskAdditions
    #[cfg(feature = "Foundation_NSURLProtocol")]
    unsafe impl NSURLProtocol {
        #[cfg(feature = "Foundation_NSURLSessionTask")]
        #[method(canInitWithTask:)]
        pub unsafe fn canInitWithTask(task: &NSURLSessionTask) -> bool;

        #[cfg(all(
            feature = "Foundation_NSCachedURLResponse",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method_id(@__retain_semantics Init initWithTask:cachedResponse:client:)]
        pub unsafe fn initWithTask_cachedResponse_client(
            this: Option<Allocated<Self>>,
            task: &NSURLSessionTask,
            cached_response: Option<&NSCachedURLResponse>,
            client: Option<&ProtocolObject<dyn NSURLProtocolClient>>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURLSessionTask")]
        #[method_id(@__retain_semantics Other task)]
        pub unsafe fn task(&self) -> Option<Id<NSURLSessionTask>>;
    }
);
