//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait BADownloadManagerDelegate: NSObjectProtocol {
        #[cfg(feature = "BADownload")]
        #[optional]
        #[method(downloadDidBegin:)]
        unsafe fn downloadDidBegin(&self, download: &BADownload);

        #[cfg(feature = "BADownload")]
        #[optional]
        #[method(downloadDidPause:)]
        unsafe fn downloadDidPause(&self, download: &BADownload);

        #[cfg(feature = "BADownload")]
        #[optional]
        #[method(download:didWriteBytes:totalBytesWritten:totalBytesExpectedToWrite:)]
        unsafe fn download_didWriteBytes_totalBytesWritten_totalBytesExpectedToWrite(
            &self,
            download: &BADownload,
            bytes_written: i64,
            total_bytes_written: i64,
            total_expected_bytes: i64,
        );

        #[cfg(all(feature = "BADownload", feature = "block2"))]
        #[optional]
        #[method(download:didReceiveChallenge:completionHandler:)]
        unsafe fn download_didReceiveChallenge_completionHandler(
            &self,
            download: &BADownload,
            challenge: &NSURLAuthenticationChallenge,
            completion_handler: &block2::Block<
                dyn Fn(NSURLSessionAuthChallengeDisposition, *mut NSURLCredential),
            >,
        );

        #[cfg(feature = "BADownload")]
        #[optional]
        #[method(download:failedWithError:)]
        unsafe fn download_failedWithError(&self, download: &BADownload, error: &NSError);

        #[cfg(feature = "BADownload")]
        #[optional]
        #[method(download:finishedWithFileURL:)]
        unsafe fn download_finishedWithFileURL(&self, download: &BADownload, file_url: &NSURL);
    }

    unsafe impl ProtocolType for dyn BADownloadManagerDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BADownloadManager;

    unsafe impl ClassType for BADownloadManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for BADownloadManager {}

extern_methods!(
    unsafe impl BADownloadManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Other sharedManager)]
        pub unsafe fn sharedManager() -> Id<BADownloadManager>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn BADownloadManagerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn BADownloadManagerDelegate>>,
        );

        #[cfg(feature = "BADownload")]
        #[method_id(@__retain_semantics Other fetchCurrentDownloads:_)]
        pub unsafe fn fetchCurrentDownloads(&self) -> Result<Id<NSArray<BADownload>>, Id<NSError>>;

        #[cfg(all(feature = "BADownload", feature = "block2"))]
        #[method(fetchCurrentDownloadsWithCompletionHandler:)]
        pub unsafe fn fetchCurrentDownloadsWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(NonNull<NSArray<BADownload>>, *mut NSError)>,
        );

        #[cfg(feature = "BADownload")]
        #[method(scheduleDownload:error:_)]
        pub unsafe fn scheduleDownload_error(
            &self,
            download: &BADownload,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "block2")]
        #[method(performWithExclusiveControl:)]
        pub unsafe fn performWithExclusiveControl(
            &self,
            perform_handler: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(performWithExclusiveControlBeforeDate:performHandler:)]
        pub unsafe fn performWithExclusiveControlBeforeDate_performHandler(
            &self,
            date: &NSDate,
            perform_handler: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(feature = "BADownload")]
        #[method(startForegroundDownload:error:_)]
        pub unsafe fn startForegroundDownload_error(
            &self,
            download: &BADownload,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "BADownload")]
        #[method(cancelDownload:error:_)]
        pub unsafe fn cancelDownload_error(&self, download: &BADownload)
            -> Result<(), Id<NSError>>;
    }
);
