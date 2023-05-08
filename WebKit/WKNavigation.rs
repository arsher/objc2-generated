//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKNavigation")]
    pub struct WKNavigation;

    #[cfg(feature = "WebKit_WKNavigation")]
    unsafe impl ClassType for WKNavigation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WKNavigation")]
unsafe impl NSObjectProtocol for WKNavigation {}

extern_methods!(
    #[cfg(feature = "WebKit_WKNavigation")]
    unsafe impl WKNavigation {
        #[method(effectiveContentMode)]
        pub unsafe fn effectiveContentMode(&self) -> WKContentMode;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WKNavigation")]
    unsafe impl WKNavigation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
