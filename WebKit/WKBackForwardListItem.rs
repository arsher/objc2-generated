//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKBackForwardListItem;

    unsafe impl ClassType for WKBackForwardListItem {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for WKBackForwardListItem {}

extern_methods!(
    unsafe impl WKBackForwardListItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other initialURL)]
        pub unsafe fn initialURL(&self) -> Retained<NSURL>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKBackForwardListItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
