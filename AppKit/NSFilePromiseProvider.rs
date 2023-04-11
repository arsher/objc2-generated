//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSFilePromiseProvider")]
    pub struct NSFilePromiseProvider;

    #[cfg(feature = "AppKit_NSFilePromiseProvider")]
    unsafe impl ClassType for NSFilePromiseProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSFilePromiseProvider")]
unsafe impl NSObjectProtocol for NSFilePromiseProvider {}

#[cfg(feature = "AppKit_NSFilePromiseProvider")]
unsafe impl NSPasteboardWriting for NSFilePromiseProvider {}

extern_methods!(
    #[cfg(feature = "AppKit_NSFilePromiseProvider")]
    unsafe impl NSFilePromiseProvider {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, file_type: &NSString);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFilePromiseProviderDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSFilePromiseProviderDelegate>>,
        );

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<Object>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&Object>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithFileType:delegate:)]
        pub unsafe fn initWithFileType_delegate(
            this: Option<Allocated<Self>>,
            file_type: &NSString,
            delegate: &ProtocolObject<dyn NSFilePromiseProviderDelegate>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSFilePromiseProviderDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSFilePromiseProvider",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other filePromiseProvider:fileNameForType:)]
        unsafe fn filePromiseProvider_fileNameForType(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            file_type: &NSString,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "AppKit_NSFilePromiseProvider",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(filePromiseProvider:writePromiseToURL:completionHandler:)]
        unsafe fn filePromiseProvider_writePromiseToURL_completionHandler(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            url: &NSURL,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "AppKit_NSFilePromiseProvider",
            feature = "Foundation_NSOperationQueue"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other operationQueueForFilePromiseProvider:)]
        unsafe fn operationQueueForFilePromiseProvider(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
        ) -> Id<NSOperationQueue>;
    }

    unsafe impl ProtocolType for dyn NSFilePromiseProviderDelegate {}
);
