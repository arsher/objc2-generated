//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLDownload")]
    pub struct NSURLDownload;

    #[cfg(feature = "Foundation_NSURLDownload")]
    unsafe impl ClassType for NSURLDownload {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSURLDownload")]
unsafe impl NSObjectProtocol for NSURLDownload {}

extern_methods!(
    #[cfg(feature = "Foundation_NSURLDownload")]
    unsafe impl NSURLDownload {
        #[cfg(feature = "Foundation_NSString")]
        #[method(canResumeDownloadDecodedWithEncodingMIMEType:)]
        pub unsafe fn canResumeDownloadDecodedWithEncodingMIMEType(mime_type: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[deprecated = "Use NSURLSession downloadTask (see NSURLSession.h)"]
        #[method_id(@__retain_semantics Init initWithRequest:delegate:)]
        pub unsafe fn initWithRequest_delegate(
            this: Option<Allocated<Self>>,
            request: &NSURLRequest,
            delegate: Option<&ProtocolObject<dyn NSURLDownloadDelegate>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSURLSession downloadTask (see NSURLSession.h)"]
        #[method_id(@__retain_semantics Init initWithResumeData:delegate:path:)]
        pub unsafe fn initWithResumeData_delegate_path(
            this: Option<Allocated<Self>>,
            resume_data: &NSData,
            delegate: Option<&ProtocolObject<dyn NSURLDownloadDelegate>>,
            path: &NSString,
        ) -> Id<Self>;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDestination:allowOverwrite:)]
        pub unsafe fn setDestination_allowOverwrite(&self, path: &NSString, allow_overwrite: bool);

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Id<NSURLRequest>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other resumeData)]
        pub unsafe fn resumeData(&self) -> Option<Id<NSData>>;

        #[method(deletesFileUponFailure)]
        pub unsafe fn deletesFileUponFailure(&self) -> bool;

        #[method(setDeletesFileUponFailure:)]
        pub unsafe fn setDeletesFileUponFailure(&self, deletes_file_upon_failure: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSURLDownload")]
    unsafe impl NSURLDownload {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSURLDownloadDelegate: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSURLDownload")]
        #[optional]
        #[method(downloadDidBegin:)]
        unsafe fn downloadDidBegin(&self, download: &NSURLDownload);

        #[cfg(all(
            feature = "Foundation_NSURLDownload",
            feature = "Foundation_NSURLRequest",
            feature = "Foundation_NSURLResponse"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other download:willSendRequest:redirectResponse:)]
        unsafe fn download_willSendRequest_redirectResponse(
            &self,
            download: &NSURLDownload,
            request: &NSURLRequest,
            redirect_response: Option<&NSURLResponse>,
        ) -> Option<Id<NSURLRequest>>;

        #[cfg(all(
            feature = "Foundation_NSURLDownload",
            feature = "Foundation_NSURLProtectionSpace"
        ))]
        #[optional]
        #[method(download:canAuthenticateAgainstProtectionSpace:)]
        unsafe fn download_canAuthenticateAgainstProtectionSpace(
            &self,
            connection: &NSURLDownload,
            protection_space: &NSURLProtectionSpace,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "Foundation_NSURLDownload"
        ))]
        #[optional]
        #[method(download:didReceiveAuthenticationChallenge:)]
        unsafe fn download_didReceiveAuthenticationChallenge(
            &self,
            download: &NSURLDownload,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(all(
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "Foundation_NSURLDownload"
        ))]
        #[optional]
        #[method(download:didCancelAuthenticationChallenge:)]
        unsafe fn download_didCancelAuthenticationChallenge(
            &self,
            download: &NSURLDownload,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(feature = "Foundation_NSURLDownload")]
        #[optional]
        #[method(downloadShouldUseCredentialStorage:)]
        unsafe fn downloadShouldUseCredentialStorage(&self, download: &NSURLDownload) -> bool;

        #[cfg(all(
            feature = "Foundation_NSURLDownload",
            feature = "Foundation_NSURLResponse"
        ))]
        #[optional]
        #[method(download:didReceiveResponse:)]
        unsafe fn download_didReceiveResponse(
            &self,
            download: &NSURLDownload,
            response: &NSURLResponse,
        );

        #[cfg(all(
            feature = "Foundation_NSURLDownload",
            feature = "Foundation_NSURLResponse"
        ))]
        #[optional]
        #[method(download:willResumeWithResponse:fromByte:)]
        unsafe fn download_willResumeWithResponse_fromByte(
            &self,
            download: &NSURLDownload,
            response: &NSURLResponse,
            starting_byte: c_longlong,
        );

        #[cfg(feature = "Foundation_NSURLDownload")]
        #[optional]
        #[method(download:didReceiveDataOfLength:)]
        unsafe fn download_didReceiveDataOfLength(
            &self,
            download: &NSURLDownload,
            length: NSUInteger,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURLDownload"))]
        #[optional]
        #[method(download:shouldDecodeSourceDataOfMIMEType:)]
        unsafe fn download_shouldDecodeSourceDataOfMIMEType(
            &self,
            download: &NSURLDownload,
            encoding_type: &NSString,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURLDownload"))]
        #[optional]
        #[method(download:decideDestinationWithSuggestedFilename:)]
        unsafe fn download_decideDestinationWithSuggestedFilename(
            &self,
            download: &NSURLDownload,
            filename: &NSString,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURLDownload"))]
        #[optional]
        #[method(download:didCreateDestination:)]
        unsafe fn download_didCreateDestination(&self, download: &NSURLDownload, path: &NSString);

        #[cfg(feature = "Foundation_NSURLDownload")]
        #[optional]
        #[method(downloadDidFinish:)]
        unsafe fn downloadDidFinish(&self, download: &NSURLDownload);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURLDownload"))]
        #[optional]
        #[method(download:didFailWithError:)]
        unsafe fn download_didFailWithError(&self, download: &NSURLDownload, error: &NSError);
    }

    unsafe impl ProtocolType for dyn NSURLDownloadDelegate {}
);
