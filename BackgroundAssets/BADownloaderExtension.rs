//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/backgroundassets/badownloaderextension?language=objc)
    pub unsafe trait BADownloaderExtension: NSObjectProtocol {
        #[cfg(all(
            feature = "BAAppExtensionInfo",
            feature = "BADownload",
            feature = "BATypes"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other downloadsForRequest:manifestURL:extensionInfo:)]
        unsafe fn downloadsForRequest_manifestURL_extensionInfo(
            &self,
            content_request: BAContentRequest,
            manifest_url: &NSURL,
            extension_info: &BAAppExtensionInfo,
        ) -> Retained<NSSet<BADownload>>;

        #[cfg(all(feature = "BADownload", feature = "block2"))]
        #[optional]
        #[method(backgroundDownload:didReceiveChallenge:completionHandler:)]
        unsafe fn backgroundDownload_didReceiveChallenge_completionHandler(
            &self,
            download: &BADownload,
            challenge: &NSURLAuthenticationChallenge,
            completion_handler: &block2::Block<
                dyn Fn(NSURLSessionAuthChallengeDisposition, *mut NSURLCredential),
            >,
        );

        #[cfg(feature = "BADownload")]
        #[optional]
        #[method(backgroundDownload:failedWithError:)]
        unsafe fn backgroundDownload_failedWithError(&self, download: &BADownload, error: &NSError);

        #[cfg(feature = "BADownload")]
        #[optional]
        #[method(backgroundDownload:finishedWithFileURL:)]
        unsafe fn backgroundDownload_finishedWithFileURL(
            &self,
            download: &BADownload,
            file_url: &NSURL,
        );

        #[deprecated = "extensionWillTerminate will not be invoked in all applicable circumstances and should not be relied upon."]
        #[optional]
        #[method(extensionWillTerminate)]
        unsafe fn extensionWillTerminate(&self);
    }

    unsafe impl ProtocolType for dyn BADownloaderExtension {}
);
