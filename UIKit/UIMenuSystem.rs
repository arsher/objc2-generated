//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenusystem?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIMenuSystem;
);

unsafe impl NSObjectProtocol for UIMenuSystem {}

extern_methods!(
    unsafe impl UIMenuSystem {
        #[method_id(@__retain_semantics Other mainSystem)]
        pub unsafe fn mainSystem(mtm: MainThreadMarker) -> Retained<UIMenuSystem>;

        #[method_id(@__retain_semantics Other contextSystem)]
        pub unsafe fn contextSystem(mtm: MainThreadMarker) -> Retained<UIMenuSystem>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(setNeedsRebuild)]
        pub unsafe fn setNeedsRebuild(&self);

        #[method(setNeedsRevalidate)]
        pub unsafe fn setNeedsRevalidate(&self);
    }
);
