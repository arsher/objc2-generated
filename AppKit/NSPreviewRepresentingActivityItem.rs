//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspreviewrepresentableactivityitem?language=objc)
    pub unsafe trait NSPreviewRepresentableActivityItem: NSObjectProtocol {
        /// The item to be shared
        #[method_id(@__retain_semantics Other item)]
        unsafe fn item(&self) -> Retained<AnyObject>;

        /// A string representing the name or title of the item to be shared
        #[optional]
        #[method_id(@__retain_semantics Other title)]
        unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// Provides an image appropriate to represent the item.
        ///
        /// This image typically is a full-size representation of the content being shared.
        /// For instance, if sharing a link to a webpage, this might be the hero image on that webpage.
        #[optional]
        #[method_id(@__retain_semantics Other imageProvider)]
        unsafe fn imageProvider(&self) -> Option<Retained<NSItemProvider>>;

        /// Provides an icon appropriate to represent the item.
        ///
        /// This icon typically is a thumbnail-sized representation of the source of the content.
        /// For instance, if sharing a link to a webpage, this might be an icon representing the website overall.
        #[optional]
        #[method_id(@__retain_semantics Other iconProvider)]
        unsafe fn iconProvider(&self) -> Option<Retained<NSItemProvider>>;
    }

    unsafe impl ProtocolType for dyn NSPreviewRepresentableActivityItem {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspreviewrepresentingactivityitem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPreviewRepresentingActivityItem;
);

unsafe impl NSObjectProtocol for NSPreviewRepresentingActivityItem {}

unsafe impl NSPreviewRepresentableActivityItem for NSPreviewRepresentingActivityItem {}

extern_methods!(
    unsafe impl NSPreviewRepresentingActivityItem {
        #[cfg(feature = "NSImage")]
        /// - Parameters:
        /// - item: The item to share
        /// - title: A title to show in a preview
        /// - image: An image to show in a preview
        /// - icon: An icon to show in a preview
        ///
        /// For more information about the parameters, see NSPreviewRepresentableActivityItem documentation
        #[method_id(@__retain_semantics Init initWithItem:title:image:icon:)]
        pub unsafe fn initWithItem_title_image_icon(
            this: Allocated<Self>,
            item: &AnyObject,
            title: Option<&NSString>,
            image: Option<&NSImage>,
            icon: Option<&NSImage>,
        ) -> Retained<Self>;

        /// - Parameters:
        /// - item: The item to share
        /// - title: A title to show in a preview
        /// - imageProvider: An NSItemProvider which provides an image to show in a preview
        /// - iconProvider: An NSItemProvider which provides an icon to show in a preview
        ///
        /// For more information about the parameters, see NSPreviewRepresentableActivityItem documentation
        #[method_id(@__retain_semantics Init initWithItem:title:imageProvider:iconProvider:)]
        pub unsafe fn initWithItem_title_imageProvider_iconProvider(
            this: Allocated<Self>,
            item: &AnyObject,
            title: Option<&NSString>,
            image_provider: Option<&NSItemProvider>,
            icon_provider: Option<&NSItemProvider>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
