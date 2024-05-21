//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// NSFileProviderThumbnailing
    #[cfg(feature = "NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method_id(@__retain_semantics Other fetchThumbnailsForItemIdentifiers:requestedSize:perThumbnailCompletionHandler:completionHandler:)]
        pub unsafe fn fetchThumbnailsForItemIdentifiers_requestedSize_perThumbnailCompletionHandler_completionHandler(
            &self,
            item_identifiers: &NSArray<NSFileProviderItemIdentifier>,
            size: CGSize,
            per_thumbnail_completion_handler: &block2::Block<
                dyn Fn(NonNull<NSFileProviderItemIdentifier>, *mut NSData, *mut NSError),
            >,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        ) -> Retained<NSProgress>;
    }
);
