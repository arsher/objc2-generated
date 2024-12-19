//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfilepromiseprovider?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFilePromiseProvider;
);

unsafe impl NSObjectProtocol for NSFilePromiseProvider {}

#[cfg(feature = "NSPasteboard")]
unsafe impl NSPasteboardWriting for NSFilePromiseProvider {}

extern_methods!(
    unsafe impl NSFilePromiseProvider {
        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Retained<NSString>;

        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, file_type: &NSString);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSFilePromiseProviderDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSFilePromiseProviderDelegate>>,
        );

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<AnyObject>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&AnyObject>);

        #[method_id(@__retain_semantics Init initWithFileType:delegate:)]
        pub unsafe fn initWithFileType_delegate(
            this: Allocated<Self>,
            file_type: &NSString,
            delegate: &ProtocolObject<dyn NSFilePromiseProviderDelegate>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFilePromiseProvider {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfilepromiseproviderdelegate?language=objc)
    pub unsafe trait NSFilePromiseProviderDelegate: NSObjectProtocol {
        #[method_id(@__retain_semantics Other filePromiseProvider:fileNameForType:)]
        unsafe fn filePromiseProvider_fileNameForType(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            file_type: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<NSString>;

        #[cfg(feature = "block2")]
        #[method(filePromiseProvider:writePromiseToURL:completionHandler:)]
        unsafe fn filePromiseProvider_writePromiseToURL_completionHandler(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            url: &NSURL,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[optional]
        #[method_id(@__retain_semantics Other operationQueueForFilePromiseProvider:)]
        unsafe fn operationQueueForFilePromiseProvider(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            mtm: MainThreadMarker,
        ) -> Retained<NSOperationQueue>;
    }

    unsafe impl ProtocolType for dyn NSFilePromiseProviderDelegate {}
);
