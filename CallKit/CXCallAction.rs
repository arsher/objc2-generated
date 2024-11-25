//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxcallaction?language=objc)
    #[unsafe(super(CXAction, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CXAction")]
    pub struct CXCallAction;
);

#[cfg(feature = "CXAction")]
unsafe impl NSCoding for CXCallAction {}

#[cfg(feature = "CXAction")]
unsafe impl NSCopying for CXCallAction {}

#[cfg(feature = "CXAction")]
unsafe impl CopyingHelper for CXCallAction {
    type Result = Self;
}

#[cfg(feature = "CXAction")]
unsafe impl NSObjectProtocol for CXCallAction {}

#[cfg(feature = "CXAction")]
unsafe impl NSSecureCoding for CXCallAction {}

extern_methods!(
    #[cfg(feature = "CXAction")]
    unsafe impl CXCallAction {
        #[method_id(@__retain_semantics Other callUUID)]
        pub unsafe fn callUUID(&self) -> Retained<NSUUID>;

        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(this: Allocated<Self>, call_uuid: &NSUUID)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CXAction")]
    unsafe impl CXCallAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
