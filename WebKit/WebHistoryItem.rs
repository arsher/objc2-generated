//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webhistoryitemchangednotification?language=objc)
    pub static WebHistoryItemChangedNotification: Option<&'static NSString>;
}

extern_class!(
    /// WebHistoryItems are created by WebKit to represent pages visited.
    /// The WebBackForwardList and WebHistory classes both use WebHistoryItems to represent
    /// pages visited.  With the exception of the displayTitle, the properties of
    /// WebHistoryItems are set by WebKit.  WebHistoryItems are normally never created directly.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/webhistoryitem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct WebHistoryItem;
);

unsafe impl NSCopying for WebHistoryItem {}

unsafe impl CopyingHelper for WebHistoryItem {
    type Result = Self;
}

unsafe impl NSObjectProtocol for WebHistoryItem {}

extern_methods!(
    unsafe impl WebHistoryItem {
        /// Parameter `URLString`: The URL string for the item.
        ///
        /// Parameter `title`: The title to use for the item.  This is normally the
        /// <title
        /// > of a page.
        ///
        /// Parameter `time`: The time used to indicate when the item was used.
        ///
        /// Initialize a new WebHistoryItem
        ///
        /// WebHistoryItems are normally created for you by the WebKit.
        /// You may use this method to prepopulate a WebBackForwardList, or create
        /// 'artificial' items to add to a WebBackForwardList.  When first initialized
        /// the URLString and originalURLString will be the same.
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithURLString:title:lastVisitedTimeInterval:)]
        pub unsafe fn initWithURLString_title_lastVisitedTimeInterval(
            this: Allocated<Self>,
            url_string: Option<&NSString>,
            title: Option<&NSString>,
            time: NSTimeInterval,
        ) -> Option<Retained<Self>>;

        /// The string representation of the initial URL of this item.
        /// This value is normally set by the WebKit.
        #[deprecated]
        #[method_id(@__retain_semantics Other originalURLString)]
        pub unsafe fn originalURLString(&self) -> Retained<NSString>;

        /// The string representation of the URL represented by this item.
        ///
        /// The URLString may be different than the originalURLString if the page
        /// redirected to a new location.  This value is normally set by the WebKit.
        #[deprecated]
        #[method_id(@__retain_semantics Other URLString)]
        pub unsafe fn URLString(&self) -> Retained<NSString>;

        /// The title of the page represented by this item.
        ///
        /// This title cannot be changed by the client.  This value
        /// is normally set by the WebKit when a page title for the item is received.
        #[deprecated]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// The last time the page represented by this item was visited. The interval
        /// is since the reference date as determined by NSDate.  This value is normally set by
        /// the WebKit.
        #[deprecated]
        #[method(lastVisitedTimeInterval)]
        pub unsafe fn lastVisitedTimeInterval(&self) -> NSTimeInterval;

        #[deprecated]
        #[method_id(@__retain_semantics Other alternateTitle)]
        pub unsafe fn alternateTitle(&self) -> Retained<NSString>;

        /// Setter for [`alternateTitle`][Self::alternateTitle].
        #[deprecated]
        #[method(setAlternateTitle:)]
        pub unsafe fn setAlternateTitle(&self, alternate_title: Option<&NSString>);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// The favorite icon of the page represented by this item.
        ///
        /// This icon returned will be determined by the WebKit.
        #[deprecated]
        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Retained<NSImage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WebHistoryItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
