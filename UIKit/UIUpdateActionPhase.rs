//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiupdateactionphase?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIUpdateActionPhase;
);

unsafe impl NSObjectProtocol for UIUpdateActionPhase {}

extern_methods!(
    unsafe impl UIUpdateActionPhase {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other afterUpdateScheduled)]
        pub unsafe fn afterUpdateScheduled(mtm: MainThreadMarker) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other beforeEventDispatch)]
        pub unsafe fn beforeEventDispatch(mtm: MainThreadMarker) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other afterEventDispatch)]
        pub unsafe fn afterEventDispatch(mtm: MainThreadMarker) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other beforeCADisplayLinkDispatch)]
        pub unsafe fn beforeCADisplayLinkDispatch(
            mtm: MainThreadMarker,
        ) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other afterCADisplayLinkDispatch)]
        pub unsafe fn afterCADisplayLinkDispatch(
            mtm: MainThreadMarker,
        ) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other beforeCATransactionCommit)]
        pub unsafe fn beforeCATransactionCommit(
            mtm: MainThreadMarker,
        ) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other afterCATransactionCommit)]
        pub unsafe fn afterCATransactionCommit(
            mtm: MainThreadMarker,
        ) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other beforeLowLatencyEventDispatch)]
        pub unsafe fn beforeLowLatencyEventDispatch(
            mtm: MainThreadMarker,
        ) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other afterLowLatencyEventDispatch)]
        pub unsafe fn afterLowLatencyEventDispatch(
            mtm: MainThreadMarker,
        ) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other beforeLowLatencyCATransactionCommit)]
        pub unsafe fn beforeLowLatencyCATransactionCommit(
            mtm: MainThreadMarker,
        ) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other afterLowLatencyCATransactionCommit)]
        pub unsafe fn afterLowLatencyCATransactionCommit(
            mtm: MainThreadMarker,
        ) -> Retained<UIUpdateActionPhase>;

        #[method_id(@__retain_semantics Other afterUpdateComplete)]
        pub unsafe fn afterUpdateComplete(mtm: MainThreadMarker) -> Retained<UIUpdateActionPhase>;
    }
);
