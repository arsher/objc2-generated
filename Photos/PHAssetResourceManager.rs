//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phassetresourcedatarequestid?language=objc)
pub type PHAssetResourceDataRequestID = i32;

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phinvalidassetresourcedatarequestid?language=objc)
pub static PHInvalidAssetResourceDataRequestID: PHAssetResourceDataRequestID = 0;

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phassetresourceprogresshandler?language=objc)
#[cfg(feature = "block2")]
pub type PHAssetResourceProgressHandler = *mut block2::Block<dyn Fn(c_double)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phassetresourcerequestoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHAssetResourceRequestOptions;
);

unsafe impl NSCopying for PHAssetResourceRequestOptions {}

unsafe impl CopyingHelper for PHAssetResourceRequestOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for PHAssetResourceRequestOptions {}

extern_methods!(
    unsafe impl PHAssetResourceRequestOptions {
        #[method(isNetworkAccessAllowed)]
        pub unsafe fn isNetworkAccessAllowed(&self) -> bool;

        /// Setter for [`isNetworkAccessAllowed`][Self::isNetworkAccessAllowed].
        #[method(setNetworkAccessAllowed:)]
        pub unsafe fn setNetworkAccessAllowed(&self, network_access_allowed: bool);

        #[cfg(feature = "block2")]
        #[method(progressHandler)]
        pub unsafe fn progressHandler(&self) -> PHAssetResourceProgressHandler;

        #[cfg(feature = "block2")]
        /// Setter for [`progressHandler`][Self::progressHandler].
        #[method(setProgressHandler:)]
        pub unsafe fn setProgressHandler(&self, progress_handler: PHAssetResourceProgressHandler);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHAssetResourceRequestOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phassetresourcemanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHAssetResourceManager;
);

unsafe impl NSObjectProtocol for PHAssetResourceManager {}

extern_methods!(
    unsafe impl PHAssetResourceManager {
        #[method_id(@__retain_semantics Other defaultManager)]
        pub unsafe fn defaultManager() -> Retained<PHAssetResourceManager>;

        #[cfg(all(feature = "PHAssetResource", feature = "block2"))]
        #[method(requestDataForAssetResource:options:dataReceivedHandler:completionHandler:)]
        pub unsafe fn requestDataForAssetResource_options_dataReceivedHandler_completionHandler(
            &self,
            resource: &PHAssetResource,
            options: Option<&PHAssetResourceRequestOptions>,
            handler: &block2::Block<dyn Fn(NonNull<NSData>)>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        ) -> PHAssetResourceDataRequestID;

        #[cfg(all(feature = "PHAssetResource", feature = "block2"))]
        #[method(writeDataForAssetResource:toFile:options:completionHandler:)]
        pub unsafe fn writeDataForAssetResource_toFile_options_completionHandler(
            &self,
            resource: &PHAssetResource,
            file_url: &NSURL,
            options: Option<&PHAssetResourceRequestOptions>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method(cancelDataRequest:)]
        pub unsafe fn cancelDataRequest(&self, request_id: PHAssetResourceDataRequestID);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHAssetResourceManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
