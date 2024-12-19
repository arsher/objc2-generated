//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkframeinfo?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKFrameInfo;
);

unsafe impl NSCopying for WKFrameInfo {}

unsafe impl CopyingHelper for WKFrameInfo {
    type Result = Self;
}

unsafe impl NSObjectProtocol for WKFrameInfo {}

extern_methods!(
    unsafe impl WKFrameInfo {
        #[method(isMainFrame)]
        pub unsafe fn isMainFrame(&self) -> bool;

        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Retained<NSURLRequest>;

        #[cfg(feature = "WKSecurityOrigin")]
        #[method_id(@__retain_semantics Other securityOrigin)]
        pub unsafe fn securityOrigin(&self) -> Retained<WKSecurityOrigin>;

        #[cfg(all(feature = "WKWebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other webView)]
        pub unsafe fn webView(&self) -> Option<Retained<WKWebView>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKFrameInfo {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
