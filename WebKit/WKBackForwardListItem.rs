//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A WKBackForwardListItem object represents a webpage in the back-forward list of a web view.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wkbackforwardlistitem?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKBackForwardListItem;
);

unsafe impl NSObjectProtocol for WKBackForwardListItem {}

extern_methods!(
    unsafe impl WKBackForwardListItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// The URL of the webpage represented by this item.
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        /// The title of the webpage represented by this item.
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// The URL of the initial request that created this item.
        #[method_id(@__retain_semantics Other initialURL)]
        pub unsafe fn initialURL(&self) -> Retained<NSURL>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKBackForwardListItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
