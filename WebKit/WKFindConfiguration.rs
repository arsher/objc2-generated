//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKFindConfiguration;

    unsafe impl ClassType for WKFindConfiguration {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSCopying for WKFindConfiguration {}

unsafe impl CopyingHelper for WKFindConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for WKFindConfiguration {}

extern_methods!(
    unsafe impl WKFindConfiguration {
        #[method(backwards)]
        pub unsafe fn backwards(&self) -> bool;

        #[method(setBackwards:)]
        pub unsafe fn setBackwards(&self, backwards: bool);

        #[method(caseSensitive)]
        pub unsafe fn caseSensitive(&self) -> bool;

        #[method(setCaseSensitive:)]
        pub unsafe fn setCaseSensitive(&self, case_sensitive: bool);

        #[method(wraps)]
        pub unsafe fn wraps(&self) -> bool;

        #[method(setWraps:)]
        pub unsafe fn setWraps(&self, wraps: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKFindConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
