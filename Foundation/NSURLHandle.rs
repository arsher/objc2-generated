//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttppropertystatuscodekey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPPropertyStatusCodeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttppropertystatusreasonkey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPPropertyStatusReasonKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttppropertyserverhttpversionkey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPPropertyServerHTTPVersionKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttppropertyredirectionheaderskey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPPropertyRedirectionHeadersKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttppropertyerrorpagedatakey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPPropertyErrorPageDataKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttppropertyhttpproxy?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPPropertyHTTPProxy: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsftppropertyuserloginkey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSFTPPropertyUserLoginKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsftppropertyuserpasswordkey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSFTPPropertyUserPasswordKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsftppropertyactivetransfermodekey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSFTPPropertyActiveTransferModeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsftppropertyfileoffsetkey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSFTPPropertyFileOffsetKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsftppropertyftpproxy?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSFTPPropertyFTPProxy: Option<&'static NSString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlhandlestatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSURLHandleStatus(pub NSUInteger);
impl NSURLHandleStatus {
    pub const NSURLHandleNotLoaded: Self = Self(0);
    pub const NSURLHandleLoadSucceeded: Self = Self(1);
    pub const NSURLHandleLoadInProgress: Self = Self(2);
    pub const NSURLHandleLoadFailed: Self = Self(3);
}

unsafe impl Encode for NSURLHandleStatus {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSURLHandleStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlhandleclient?language=objc)
    #[deprecated]
    pub unsafe trait NSURLHandleClient {
        #[cfg(feature = "NSData")]
        #[deprecated]
        #[method(URLHandle:resourceDataDidBecomeAvailable:)]
        unsafe fn URLHandle_resourceDataDidBecomeAvailable(
            &self,
            sender: Option<&NSURLHandle>,
            new_bytes: Option<&NSData>,
        );

        #[deprecated]
        #[method(URLHandleResourceDidBeginLoading:)]
        unsafe fn URLHandleResourceDidBeginLoading(&self, sender: Option<&NSURLHandle>);

        #[deprecated]
        #[method(URLHandleResourceDidFinishLoading:)]
        unsafe fn URLHandleResourceDidFinishLoading(&self, sender: Option<&NSURLHandle>);

        #[deprecated]
        #[method(URLHandleResourceDidCancelLoading:)]
        unsafe fn URLHandleResourceDidCancelLoading(&self, sender: Option<&NSURLHandle>);

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method(URLHandle:resourceDidFailLoadingWithReason:)]
        unsafe fn URLHandle_resourceDidFailLoadingWithReason(
            &self,
            sender: Option<&NSURLHandle>,
            reason: Option<&NSString>,
        );
    }

    unsafe impl ProtocolType for dyn NSURLHandleClient {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlhandle?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLHandle;
);

unsafe impl NSObjectProtocol for NSURLHandle {}

extern_methods!(
    unsafe impl NSURLHandle {
        #[deprecated]
        #[method(registerURLHandleClass:)]
        pub unsafe fn registerURLHandleClass(an_url_handle_subclass: Option<&AnyClass>);

        #[cfg(feature = "NSURL")]
        #[deprecated]
        #[method(URLHandleClassForURL:)]
        pub unsafe fn URLHandleClassForURL(an_url: Option<&NSURL>) -> Option<&'static AnyClass>;

        #[deprecated]
        #[method(status)]
        pub unsafe fn status(&self) -> NSURLHandleStatus;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other failureReason)]
        pub unsafe fn failureReason(&self) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method(addClient:)]
        pub unsafe fn addClient(&self, client: Option<&ProtocolObject<dyn NSURLHandleClient>>);

        #[deprecated]
        #[method(removeClient:)]
        pub unsafe fn removeClient(&self, client: Option<&ProtocolObject<dyn NSURLHandleClient>>);

        #[deprecated]
        #[method(loadInBackground)]
        pub unsafe fn loadInBackground(&self);

        #[deprecated]
        #[method(cancelLoadInBackground)]
        pub unsafe fn cancelLoadInBackground(&self);

        #[cfg(feature = "NSData")]
        #[deprecated]
        #[method_id(@__retain_semantics Other resourceData)]
        pub unsafe fn resourceData(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "NSData")]
        #[deprecated]
        #[method_id(@__retain_semantics Other availableResourceData)]
        pub unsafe fn availableResourceData(&self) -> Option<Retained<NSData>>;

        #[deprecated]
        #[method(expectedResourceDataSize)]
        pub unsafe fn expectedResourceDataSize(&self) -> c_longlong;

        #[deprecated]
        #[method(flushCachedData)]
        pub unsafe fn flushCachedData(&self);

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method(backgroundLoadDidFailWithReason:)]
        pub unsafe fn backgroundLoadDidFailWithReason(&self, reason: Option<&NSString>);

        #[cfg(feature = "NSData")]
        #[deprecated]
        #[method(didLoadBytes:loadComplete:)]
        pub unsafe fn didLoadBytes_loadComplete(&self, new_bytes: Option<&NSData>, yorn: bool);

        #[cfg(feature = "NSURL")]
        #[deprecated]
        #[method(canInitWithURL:)]
        pub unsafe fn canInitWithURL(an_url: Option<&NSURL>) -> bool;

        #[cfg(feature = "NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Other cachedHandleForURL:)]
        pub unsafe fn cachedHandleForURL(an_url: Option<&NSURL>) -> Option<Retained<NSURLHandle>>;

        #[cfg(feature = "NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithURL:cached:)]
        pub unsafe fn initWithURL_cached(
            this: Allocated<Self>,
            an_url: Option<&NSURL>,
            will_cache: bool,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other propertyForKey:)]
        pub unsafe fn propertyForKey(
            &self,
            property_key: Option<&NSString>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other propertyForKeyIfAvailable:)]
        pub unsafe fn propertyForKeyIfAvailable(
            &self,
            property_key: Option<&NSString>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method(writeProperty:forKey:)]
        pub unsafe fn writeProperty_forKey(
            &self,
            property_value: Option<&AnyObject>,
            property_key: Option<&NSString>,
        ) -> bool;

        #[cfg(feature = "NSData")]
        #[deprecated]
        #[method(writeData:)]
        pub unsafe fn writeData(&self, data: Option<&NSData>) -> bool;

        #[cfg(feature = "NSData")]
        #[deprecated]
        #[method_id(@__retain_semantics Other loadInForeground)]
        pub unsafe fn loadInForeground(&self) -> Option<Retained<NSData>>;

        #[deprecated]
        #[method(beginLoadInBackground)]
        pub unsafe fn beginLoadInBackground(&self);

        #[deprecated]
        #[method(endLoadInBackground)]
        pub unsafe fn endLoadInBackground(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSURLHandle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
