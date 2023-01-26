//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileCoordinatorReadingOptions {
        NSFileCoordinatorReadingWithoutChanges = 1 << 0,
        NSFileCoordinatorReadingResolvesSymbolicLink = 1 << 1,
        NSFileCoordinatorReadingImmediatelyAvailableMetadataOnly = 1 << 2,
        NSFileCoordinatorReadingForUploading = 1 << 3,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileCoordinatorWritingOptions {
        NSFileCoordinatorWritingForDeleting = 1 << 0,
        NSFileCoordinatorWritingForMoving = 1 << 1,
        NSFileCoordinatorWritingForMerging = 1 << 2,
        NSFileCoordinatorWritingForReplacing = 1 << 3,
        NSFileCoordinatorWritingContentIndependentMetadataOnly = 1 << 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFileAccessIntent")]
    pub struct NSFileAccessIntent;

    #[cfg(feature = "Foundation_NSFileAccessIntent")]
    unsafe impl ClassType for NSFileAccessIntent {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSFileAccessIntent")]
unsafe impl NSObjectProtocol for NSFileAccessIntent {}

extern_methods!(
    #[cfg(feature = "Foundation_NSFileAccessIntent")]
    unsafe impl NSFileAccessIntent {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other readingIntentWithURL:options:)]
        pub unsafe fn readingIntentWithURL_options(
            url: &NSURL,
            options: NSFileCoordinatorReadingOptions,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other writingIntentWithURL:options:)]
        pub unsafe fn writingIntentWithURL_options(
            url: &NSURL,
            options: NSFileCoordinatorWritingOptions,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFileCoordinator")]
    pub struct NSFileCoordinator;

    #[cfg(feature = "Foundation_NSFileCoordinator")]
    unsafe impl ClassType for NSFileCoordinator {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSFileCoordinator")]
unsafe impl NSObjectProtocol for NSFileCoordinator {}

extern_methods!(
    #[cfg(feature = "Foundation_NSFileCoordinator")]
    unsafe impl NSFileCoordinator {
        #[method(addFilePresenter:)]
        pub unsafe fn addFilePresenter(file_presenter: &ProtocolObject<dyn NSFilePresenter>);

        #[method(removeFilePresenter:)]
        pub unsafe fn removeFilePresenter(file_presenter: &ProtocolObject<dyn NSFilePresenter>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other filePresenters)]
        pub unsafe fn filePresenters() -> Id<NSArray<ProtocolObject<dyn NSFilePresenter>>, Shared>;

        #[method_id(@__retain_semantics Init initWithFilePresenter:)]
        pub unsafe fn initWithFilePresenter(
            this: Option<Allocated<Self>>,
            file_presenter_or_nil: Option<&ProtocolObject<dyn NSFilePresenter>>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other purposeIdentifier)]
        pub unsafe fn purposeIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPurposeIdentifier:)]
        pub unsafe fn setPurposeIdentifier(&self, purpose_identifier: &NSString);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSFileAccessIntent",
            feature = "Foundation_NSOperationQueue"
        ))]
        #[method(coordinateAccessWithIntents:queue:byAccessor:)]
        pub unsafe fn coordinateAccessWithIntents_queue_byAccessor(
            &self,
            intents: &NSArray<NSFileAccessIntent>,
            queue: &NSOperationQueue,
            accessor: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(coordinateReadingItemAtURL:options:error:byAccessor:)]
        pub unsafe fn coordinateReadingItemAtURL_options_error_byAccessor(
            &self,
            url: &NSURL,
            options: NSFileCoordinatorReadingOptions,
            out_error: *mut *mut NSError,
            reader: &Block<(NonNull<NSURL>,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(coordinateWritingItemAtURL:options:error:byAccessor:)]
        pub unsafe fn coordinateWritingItemAtURL_options_error_byAccessor(
            &self,
            url: &NSURL,
            options: NSFileCoordinatorWritingOptions,
            out_error: *mut *mut NSError,
            writer: &Block<(NonNull<NSURL>,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(coordinateReadingItemAtURL:options:writingItemAtURL:options:error:byAccessor:)]
        pub unsafe fn coordinateReadingItemAtURL_options_writingItemAtURL_options_error_byAccessor(
            &self,
            reading_url: &NSURL,
            reading_options: NSFileCoordinatorReadingOptions,
            writing_url: &NSURL,
            writing_options: NSFileCoordinatorWritingOptions,
            out_error: *mut *mut NSError,
            reader_writer: &Block<(NonNull<NSURL>, NonNull<NSURL>), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(coordinateWritingItemAtURL:options:writingItemAtURL:options:error:byAccessor:)]
        pub unsafe fn coordinateWritingItemAtURL_options_writingItemAtURL_options_error_byAccessor(
            &self,
            url1: &NSURL,
            options1: NSFileCoordinatorWritingOptions,
            url2: &NSURL,
            options2: NSFileCoordinatorWritingOptions,
            out_error: *mut *mut NSError,
            writer: &Block<(NonNull<NSURL>, NonNull<NSURL>), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(prepareForReadingItemsAtURLs:options:writingItemsAtURLs:options:error:byAccessor:)]
        pub unsafe fn prepareForReadingItemsAtURLs_options_writingItemsAtURLs_options_error_byAccessor(
            &self,
            reading_ur_ls: &NSArray<NSURL>,
            reading_options: NSFileCoordinatorReadingOptions,
            writing_ur_ls: &NSArray<NSURL>,
            writing_options: NSFileCoordinatorWritingOptions,
            out_error: *mut *mut NSError,
            batch_accessor: &Block<(NonNull<Block<(), ()>>,), ()>,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method(itemAtURL:willMoveToURL:)]
        pub unsafe fn itemAtURL_willMoveToURL(&self, old_url: &NSURL, new_url: &NSURL);

        #[cfg(feature = "Foundation_NSURL")]
        #[method(itemAtURL:didMoveToURL:)]
        pub unsafe fn itemAtURL_didMoveToURL(&self, old_url: &NSURL, new_url: &NSURL);

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSURL"))]
        #[method(itemAtURL:didChangeUbiquityAttributes:)]
        pub unsafe fn itemAtURL_didChangeUbiquityAttributes(
            &self,
            url: &NSURL,
            attributes: &NSSet<NSURLResourceKey>,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);
