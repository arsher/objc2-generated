//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// MPPlayableContentDataSource is a protocol that application objects conform to
    /// if they want to support external media players, such as vehicle head units.
    /// Data sources are responsible for providing metadata about your media to these
    /// systems in a meaningful way, so that features like user interfaces and play
    /// queues can be setup automatically.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpplayablecontentdatasource?language=objc)
    #[deprecated = "Use CarPlay framework"]
    pub unsafe trait MPPlayableContentDataSource: NSObjectProtocol {
        #[cfg(feature = "block2")]
        /// Tells the data source to begin loading content items that are children of the
        /// item specified by indexPath. This is provided so that applications can begin
        /// asynchronous batched loading of content before MediaPlayer begins asking for
        /// content items to display.
        /// Client applications should always call the completion handler after loading
        /// has finished, if this method is implemented.
        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(beginLoadingChildItemsAtIndexPath:completionHandler:)]
        unsafe fn beginLoadingChildItemsAtIndexPath_completionHandler(
            &self,
            index_path: &NSIndexPath,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        /// Tells MediaPlayer whether the content provided by the data source supports
        /// playback progress as a property of its metadata.
        /// If this method is not implemented, MediaPlayer will assume that progress is
        /// not supported for any content items.
        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(childItemsDisplayPlaybackProgressAtIndexPath:)]
        unsafe fn childItemsDisplayPlaybackProgressAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> bool;

        #[cfg(all(feature = "MPContentItem", feature = "block2"))]
        /// Provides a content item for the provided identifier.
        /// Provide nil if there is no content item corresponding to the identifier.
        /// Provide an error if there is an error that will not allow content items
        /// to be retrieved.
        /// Client applications should always call the completion handler after loading
        /// has finished, if this method is implemented.
        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(contentItemForIdentifier:completionHandler:)]
        unsafe fn contentItemForIdentifier_completionHandler(
            &self,
            identifier: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut MPContentItem, *mut NSError)>,
        );

        /// Returns the number of child nodes at the specified index path. In a virtual
        /// filesystem, this would be the number of files in a specific folder. An empty
        /// index path represents the root node.
        #[deprecated = "Use CarPlay framework"]
        #[method(numberOfChildItemsAtIndexPath:)]
        unsafe fn numberOfChildItemsAtIndexPath(&self, index_path: &NSIndexPath) -> NSInteger;

        #[cfg(feature = "MPContentItem")]
        /// Returns the content item at the specified index path. If the content item is
        /// mutated after returning, its updated contents will be sent to MediaPlayer.
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other contentItemAtIndexPath:)]
        unsafe fn contentItemAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<MPContentItem>>;
    }

    unsafe impl ProtocolType for dyn MPPlayableContentDataSource {}
);
