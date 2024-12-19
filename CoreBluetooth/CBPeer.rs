//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbpeer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CBPeer;
);

unsafe impl NSCopying for CBPeer {}

unsafe impl CopyingHelper for CBPeer {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CBPeer {}

extern_methods!(
    unsafe impl CBPeer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSUUID>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CBPeer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
