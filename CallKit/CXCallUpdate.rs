//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxcallupdate?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXCallUpdate;
);

unsafe impl NSCopying for CXCallUpdate {}

unsafe impl CopyingHelper for CXCallUpdate {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CXCallUpdate {}

extern_methods!(
    unsafe impl CXCallUpdate {
        #[cfg(feature = "CXHandle")]
        #[method_id(@__retain_semantics Other remoteHandle)]
        pub unsafe fn remoteHandle(&self) -> Option<Retained<CXHandle>>;

        #[cfg(feature = "CXHandle")]
        #[method(setRemoteHandle:)]
        pub unsafe fn setRemoteHandle(&self, remote_handle: Option<&CXHandle>);

        #[method_id(@__retain_semantics Other localizedCallerName)]
        pub unsafe fn localizedCallerName(&self) -> Option<Retained<NSString>>;

        #[method(setLocalizedCallerName:)]
        pub unsafe fn setLocalizedCallerName(&self, localized_caller_name: Option<&NSString>);

        #[method(supportsHolding)]
        pub unsafe fn supportsHolding(&self) -> bool;

        #[method(setSupportsHolding:)]
        pub unsafe fn setSupportsHolding(&self, supports_holding: bool);

        #[method(supportsGrouping)]
        pub unsafe fn supportsGrouping(&self) -> bool;

        #[method(setSupportsGrouping:)]
        pub unsafe fn setSupportsGrouping(&self, supports_grouping: bool);

        #[method(supportsUngrouping)]
        pub unsafe fn supportsUngrouping(&self) -> bool;

        #[method(setSupportsUngrouping:)]
        pub unsafe fn setSupportsUngrouping(&self, supports_ungrouping: bool);

        #[method(supportsDTMF)]
        pub unsafe fn supportsDTMF(&self) -> bool;

        #[method(setSupportsDTMF:)]
        pub unsafe fn setSupportsDTMF(&self, supports_dtmf: bool);

        #[method(hasVideo)]
        pub unsafe fn hasVideo(&self) -> bool;

        #[method(setHasVideo:)]
        pub unsafe fn setHasVideo(&self, has_video: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CXCallUpdate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
