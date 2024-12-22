//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A WKProcessPool object represents a pool of web content processes.
    /// The process pool associated with a web view is specified by its web view
    /// configuration. Each web view is given its own web content process until an
    /// implementation-defined process limit is reached; after that, web views
    /// with the same process pool end up sharing web content processes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wkprocesspool?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKProcessPool;
);

unsafe impl NSCoding for WKProcessPool {}

unsafe impl NSObjectProtocol for WKProcessPool {}

unsafe impl NSSecureCoding for WKProcessPool {}

extern_methods!(
    unsafe impl WKProcessPool {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKProcessPool {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
