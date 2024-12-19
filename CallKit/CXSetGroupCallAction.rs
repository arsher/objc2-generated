//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxsetgroupcallaction?language=objc)
    #[unsafe(super(CXCallAction, CXAction, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    pub struct CXSetGroupCallAction;
);

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSCoding for CXSetGroupCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSCopying for CXSetGroupCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl CopyingHelper for CXSetGroupCallAction {
    type Result = Self;
}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSObjectProtocol for CXSetGroupCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSSecureCoding for CXSetGroupCallAction {}

extern_methods!(
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXSetGroupCallAction {
        #[method_id(@__retain_semantics Init initWithCallUUID:callUUIDToGroupWith:)]
        pub unsafe fn initWithCallUUID_callUUIDToGroupWith(
            this: Allocated<Self>,
            call_uuid: &NSUUID,
            call_uuid_to_group_with: Option<&NSUUID>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(this: Allocated<Self>, call_uuid: &NSUUID)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Other callUUIDToGroupWith)]
        pub unsafe fn callUUIDToGroupWith(&self) -> Option<Retained<NSUUID>>;

        #[method(setCallUUIDToGroupWith:)]
        pub unsafe fn setCallUUIDToGroupWith(&self, call_uuid_to_group_with: Option<&NSUUID>);
    }
);

extern_methods!(
    /// Methods declared on superclass `CXCallAction`
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXSetGroupCallAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXSetGroupCallAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
